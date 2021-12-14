use crate::*;

pub trait AccountWithWithdrawal: Account {
    fn get_withdrawal_type(&self) -> WithdrawalOptions;
    fn get_withdrawal_value(&self) -> f64 {
        match self.get_withdrawal_type() {
            WithdrawalOptions::Fixed => todo!(),
            WithdrawalOptions::FixedWithInflation => todo!(),
            WithdrawalOptions::EndAtZero => todo!(),
            WithdrawalOptions::ColFracOfSavings => todo!(),
        }
    }
}
