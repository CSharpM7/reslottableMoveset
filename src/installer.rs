#[smashline::installer]
pub fn installer() {
    install();
}

pub fn install() {
    crate::sonic::install();
}