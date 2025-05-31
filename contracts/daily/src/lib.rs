use solana_program::{
    account_info::{AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
    system_instruction,
    program::invoke,
    program::invoke_signed,
    sysvar::Sysvar,
    sysvar::rent::Rent,
    clock::Clock,
    hash::{Hash, Hasher},
};

use std::str::FromStr;

entrypoint!(process_instruction);

const RECORD_SIZE: usize = 32;
const MIN_PLAYERS: u64 = 5;

fn get_random_number(players: u64, program_id: &Pubkey) -> u64 {
    let clock = Clock::get().unwrap();

    // 
    let timestamp_ms = clock.unix_timestamp * 1000 + clock.slot as i64;  // 

    // 
    let mut hasher = Hasher::default();
    hasher.hash(timestamp_ms.to_le_bytes().as_ref());
    hasher.hash(program_id.as_ref());

    let hash_result: Hash = hasher.result();
    let hash_bytes = hash_result.to_bytes(); // 

    // 
    let random_number = u64::from_le_bytes([
        hash_bytes[0], hash_bytes[1], hash_bytes[2], hash_bytes[3], 
        hash_bytes[4], hash_bytes[5], hash_bytes[6], hash_bytes[7]
    ]);

    // 
    (random_number % players) + 1
}

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("INIT");

    let storage_account = &accounts[0];  // 
    let pda_account = &accounts[2];

    if storage_account.owner != _program_id {
        msg!("State account is not owned by the program.");
        return Err(ProgramError::IllegalOwner);
    }

    let seed = "pda_daily";
    let (derived_pda, _bump_seed) = Pubkey::find_program_address(&[seed.as_bytes()], _program_id);

    if derived_pda != *pda_account.key {
        msg!("Contract account its not the contract account");
        return Err(ProgramError::IncorrectProgramId);
    }

    let operation = instruction_data[0];
    msg!("Operation: {}", operation);
    match operation {
        1 => add(_program_id, accounts),
        2 => config(_program_id, accounts, &instruction_data[1..]),
        3 => resize(_program_id, accounts),
        4 => cash_out(_program_id, accounts),
        _ => Err(ProgramError::InvalidInstructionData)
    }
}

fn add(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let storage_account = &accounts[0];
    let sender_account = &accounts[1];
    let pda_account = &accounts[2];    
    let system_program = &accounts[3];
    let owner_account = &accounts[4];
    let burn_account = &accounts[5];
    
    let expected_burn_account = Pubkey::from_str("J58BckXJeu5Zfhv482WjnpypUtAAKnVqeFYQgKvxDZhU")
        .expect("Invalid address");
    
    if *burn_account.key != expected_burn_account {
        return Err(ProgramError::Custom(1));
    }

    let expected_owner_account = Pubkey::from_str("4W47DbSDBqdKpWraMeg7wED4ExumuQqwWhsGz14L4UXS")
        .expect("Invalid address");
    
    if *owner_account.key != expected_owner_account {
        return Err(ProgramError::Custom(2));   
    }

    let mut storage_data = storage_account.try_borrow_mut_data()?;

    let previous_balance = u64::from_le_bytes(storage_data[0..8].try_into().unwrap());
    msg!("Previous balance: {}", previous_balance);

    let previous_play = u64::from_le_bytes(storage_data[8..16].try_into().unwrap());
    msg!("Previous play: {}", previous_play);

    let ticket_price = u64::from_le_bytes(storage_data[16..24].try_into().unwrap());
    msg!("Ticket price: {}", ticket_price);

    let timelapse = u64::from_le_bytes(storage_data[24..32].try_into().unwrap());
    msg!("Time lapse: {}", timelapse);

    let mut current_balance = pda_account.lamports();
    msg!("Current balance: {}", current_balance);

    let last_winner_jackpot = u64::from_le_bytes(storage_data[32..40].try_into().unwrap());
    msg!("Last winner jackpot: {}", last_winner_jackpot);
    
    let last_winner_address = Pubkey::new_from_array(storage_data[40..72].try_into().unwrap());
    msg!("Last winner address: {}", last_winner_address);

    let last_winner_charged = storage_data[72] as u64;
    msg!("Last winner charged: {}", last_winner_charged);

    let input_amount = current_balance - previous_balance;
    if input_amount < ticket_price {
        return Err(ProgramError::Custom(3));
    }
    msg!("Amount transferred: {}", input_amount);

    storage_data[0..8].copy_from_slice(&current_balance.to_le_bytes());

    let current_play = Clock::get().map_err(|_| ProgramError::InvalidAccountData)?.unix_timestamp;

    msg!("previous date: {:?}, current date: {:?})", previous_play, current_play);
    
    if (previous_play as i64) + (timelapse as i64) < current_play {
        msg!("New game (previous: {:?}, current: {:?})", previous_play, current_play);

        let storage_data_ff = &mut storage_data[73..];
        let mut players:u64 = 0;
        for record in storage_data_ff.chunks_exact_mut(RECORD_SIZE) {
            if record.iter().all(|&x| x == 0) {
                //
            }else{
                players = players + 1;
            }
        }
        msg!("Players: {}", players);      

        if players >= MIN_PLAYERS - 1 {

            let mut amount = ((current_balance - input_amount) / 100) * 80;  

            if last_winner_charged == 0 {
                amount = ((current_balance - last_winner_jackpot - input_amount) / 100) * 80;  
            }
          
            msg!("Amount for claim: {}", amount);

            let mut amount_burn = ((current_balance - input_amount) / 100) * 10;

            if last_winner_charged == 0 {
                amount_burn = ((current_balance - last_winner_jackpot - input_amount) / 100) * 10;  
            }

            msg!("Amount for burn: {}", amount_burn);

            let mut amount_team = ((current_balance - input_amount) / 100) * 10;

            if last_winner_charged == 0 {
                amount_team = ((current_balance - last_winner_jackpot - input_amount) / 100) * 10;  
            }

            msg!("Amount for team: {}", amount_team);

            let transfer_instruction_burn = system_instruction::transfer(
                &pda_account.key,
                &burn_account.key,
                amount_burn,
            );
            current_balance = current_balance - amount_burn;

            let seed = b"pda_daily";
            let (pda, bump) = Pubkey::find_program_address(&[seed], &_program_id);
            println!("PDA: {}, Bump: {}", pda, bump);
            let seeds = &[b"pda_daily".as_ref(), &[bump]];

            invoke_signed(
                &transfer_instruction_burn,
                &[pda_account.clone(), burn_account.clone(), system_program.clone()],
                &[seeds],
            )?;
            msg!("burn transferred");
            let transfer_instruction_team = system_instruction::transfer(
                &pda_account.key,
                &owner_account.key,
                amount_team,
            );
            current_balance = current_balance - amount_team;

            let seed = b"pda_daily";
            let (pda, bump) = Pubkey::find_program_address(&[seed], &_program_id);
            println!("PDA: {}, Bump: {}", pda, bump);
            let seeds = &[b"pda_daily".as_ref(), &[bump]];

            invoke_signed(
                &transfer_instruction_team,
                &[pda_account.clone(), owner_account.clone(), system_program.clone()],
                &[seeds],
            )?;    
            msg!("team transferred");

            if last_winner_charged == 0 {
                if last_winner_jackpot > 0 {
                    msg!("burn the uncollected jackpot");
                    let transfer_instruction_burn = system_instruction::transfer(
                        &pda_account.key,
                        &burn_account.key,
                        last_winner_jackpot,
                    );
                    current_balance = current_balance - last_winner_jackpot;

                    let seed = b"pda_daily";
                    let (pda, bump) = Pubkey::find_program_address(&[seed], &_program_id);
                    println!("PDA: {}, Bump: {}", pda, bump);
                    let seeds = &[b"pda_daily".as_ref(), &[bump]];

                    invoke_signed(
                        &transfer_instruction_burn,
                        &[pda_account.clone(), burn_account.clone(), system_program.clone()],
                        &[seeds],
                    )?;
                    msg!("burn transferred");

                }
            }

            let winner: u64 = get_random_number(players, _program_id);
            let mut player = 0;

            for record in storage_data_ff.chunks_exact_mut(RECORD_SIZE) {
                player = player + 1;
                if player == winner {

                    let winner_address = Pubkey::new_from_array(record[..RECORD_SIZE].try_into().unwrap());

                    storage_data[32..40].copy_from_slice(&amount.to_le_bytes());

                    storage_data[40..72].copy_from_slice(winner_address.as_ref());

                    storage_data[72..73].fill(0);

                    storage_data[73..].fill(0);

                    storage_data[8..16].copy_from_slice(&current_play.to_le_bytes());

                    break;
                }
            }

        }else{
            msg!("No players for random");
        }

        storage_data[0..8].copy_from_slice(&current_balance.to_le_bytes());

    }else{
        msg!("No new game (previous: {:?}, current: {:?})", previous_play, current_play);
    }

    let storage_data_ff = &mut storage_data[73..]; //skip balance storage area

    for record in storage_data_ff.chunks_exact_mut(RECORD_SIZE) {
        if record[..RECORD_SIZE] == sender_account.key.as_ref()[..RECORD_SIZE] {
            msg!("Address already exists");
            return Err(ProgramError::Custom(4));
        }
    }

    for record in storage_data_ff.chunks_exact_mut(RECORD_SIZE) {
        if record.iter().all(|&x| x == 0) {
            record.copy_from_slice(sender_account.key.as_ref());
            msg!("Address added successfully.");
            return Ok(());
        }
    }

    msg!("No space left.");
    return Err(ProgramError::Custom(5));    

}

fn config(_program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let storage_account = &accounts[0];
    let sender_account = &accounts[1];

    let expected_sender_account = Pubkey::from_str("4W47DbSDBqdKpWraMeg7wED4ExumuQqwWhsGz14L4UXS")
        .expect("Invalid address");
    
    if *sender_account.key != expected_sender_account {
        return Err(ProgramError::Custom(6));    
    }

    if !sender_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut storage_data = storage_account.try_borrow_mut_data()?;

    let ticket_price = u64::from_le_bytes(storage_data[16..24].try_into().unwrap());
    msg!("Current ticket price: {}", ticket_price);

    let timelapse = u64::from_le_bytes(storage_data[24..32].try_into().unwrap());
    msg!("Current time lapse: {}", timelapse);

    const NULL_PRICE: [u8; 8] = [0; 8];
    let new_ticket_price = &data[..8];
    let new_timelapse = &data[8..16];

    if new_ticket_price == NULL_PRICE {
        msg!("Price null");
        return Err(ProgramError::Custom(7));
    }
    msg!("New ticket price: {:?}", new_ticket_price);
    msg!("New time lapse: {:?}", new_timelapse);

    storage_data[16..24].copy_from_slice(&new_ticket_price);
    storage_data[24..32].copy_from_slice(&new_timelapse);

    Ok(())
}

fn resize(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let storage_account = &accounts[0];
    let sender_account = &accounts[1];
    let system_program = &accounts[3];

    let expected_sender_account = Pubkey::from_str("4W47DbSDBqdKpWraMeg7wED4ExumuQqwWhsGz14L4UXS")
        .expect("Invalid address");
    
    if *sender_account.key != expected_sender_account {
        return Err(ProgramError::Custom(6));    
    }

    if !sender_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    let new_size = storage_account.data.borrow().len() + (RECORD_SIZE * 100);

    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(new_size);

    let lamports_diff = new_minimum_balance.saturating_sub(storage_account.lamports());
    invoke(
      &system_instruction::transfer(sender_account.key, storage_account.key, lamports_diff),
      &[
          sender_account.clone(),
          storage_account.clone(),
          system_program.clone(),
      ],
    )?;

    storage_account.realloc(new_size, false)?;

    Ok(())
}

fn cash_out(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let storage_account = &accounts[0];
    let sender_account = &accounts[1];
    let pda_account = &accounts[2];    
    let system_program = &accounts[3];
    let owner_account = &accounts[4];
    let burn_account = &accounts[5];
    
    let expected_burn_account = Pubkey::from_str("J58BckXJeu5Zfhv482WjnpypUtAAKnVqeFYQgKvxDZhU")
        .expect("Invalid address");
    
    if *burn_account.key != expected_burn_account {
        return Err(ProgramError::Custom(8));
    }

    let expected_owner_account = Pubkey::from_str("4W47DbSDBqdKpWraMeg7wED4ExumuQqwWhsGz14L4UXS")
        .expect("Invalid address");
    
    if *owner_account.key != expected_owner_account {
        return Err(ProgramError::Custom(9));    
    }

    if !sender_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    let mut current_balance = pda_account.lamports();
    msg!("Current balance: {}", current_balance);
    
    let mut storage_data = storage_account.try_borrow_mut_data()?;

    let last_winner_jackpot = u64::from_le_bytes(storage_data[32..40].try_into().unwrap());
    msg!("Last winner jackpot: {}", last_winner_jackpot);
    
    let last_winner_address = Pubkey::new_from_array(storage_data[40..72].try_into().unwrap());
    msg!("Last winner address: {}", last_winner_address);

    let last_winner_charged = storage_data[72] as u64;
    msg!("Last winner charged: {}", last_winner_charged);

    if last_winner_charged == 0 {
        msg!("The prize has not yet been collected");
        
        if *sender_account.key != last_winner_address {
            return Err(ProgramError::Custom(10));    
        }

        let transfer_instruction_winner = system_instruction::transfer(
            &pda_account.key,
            &sender_account.key,
            last_winner_jackpot,
        );

        let seed = b"pda_daily";
        let (pda, bump) = Pubkey::find_program_address(&[seed], &_program_id);
        println!("PDA: {}, Bump: {}", pda, bump);
        let seeds = &[b"pda_daily".as_ref(), &[bump]];

        invoke_signed(
            &transfer_instruction_winner,
            &[pda_account.clone(), sender_account.clone(), system_program.clone()],
            &[seeds],
        )?;
        
        msg!("player transferred");

        storage_data[72..73].fill(1);
        current_balance = current_balance - last_winner_jackpot;     
        storage_data[0..8].copy_from_slice(&current_balance.to_le_bytes());

    }else{
        msg!("The prize has already been collected");
        return Err(ProgramError::IncorrectProgramId);
    }

    Ok(())
}