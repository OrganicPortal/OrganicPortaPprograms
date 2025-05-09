use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("SeedInfo Program entrypoint");

    let accounts_iter = &mut accounts.iter();
    let data_account = next_account_info(accounts_iter)?;

    // Перевіряємо чи це акаунт, що належить нашій програмі
    if data_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Отримуємо дані акаунту
    let mut data = data_account.data.borrow_mut();

    // Отримуємо загальний розмір простору
    let space = data.len();

    // Виділяємо перші 4 байти для зберігання довжини
    if space < 4 {
        return Err(ProgramError::AccountDataTooSmall);
    }

    // Отримуємо поточну довжину даних
    let curent_len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;

    // Отримуємо довжину даних інструкції
    let new_data_len = instruction_data.len();

    // Отримуємо очікуваний розмір даних
    let requaired_len = 4 + curent_len + new_data_len;

    // Якщо довжини акаунту не достатньо
    if requaired_len > space {
        return Err(ProgramError::AccountDataTooSmall);
    }

    // Записуємо нові дані
    let start = 4 + curent_len;
    data[start..start + new_data_len].copy_from_slice(&instruction_data);

    // Записуємо довжину
    let new_data_len = curent_len + new_data_len;
    data[0..4].copy_from_slice(&(new_data_len as u32).to_le_bytes());

    msg!("SeedInfo записано, довжина: {}", 0);
    Ok(())
}
