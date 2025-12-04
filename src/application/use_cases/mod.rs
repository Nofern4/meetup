pub mod authentication;
pub mod brawlers;
pub mod crew_operation;
pub mod mission_management;
pub mod mission_operation;
pub mod mission_viewing;

//pub(crate) กำหนด ระดับการเข้าถึง (visibility) ของ module (mod) ให้สามารถเข้าถึงได้ เฉพาะภายใน crate เดียวกัน เท่านั้น
pub(crate) mod brawlers_test;
pub(crate) mod crew_operation_test;
pub(crate) mod mission_management_test;
pub(crate) mod mission_operation_test;
pub(crate) mod mission_viewing_test;
