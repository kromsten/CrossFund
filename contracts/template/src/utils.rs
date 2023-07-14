use crate::storage::{Application, GoodFee};


pub fn valid_application(
    application: &Application
) -> bool {
    // TODO: implement
    let sh =  shareholders(application);
    sh.len() < 100 && sh.iter().map(|f| f.percent_share).sum::<u8>() == 1
}


pub fn shareholders(
    application: &Application
) -> Vec<GoodFee> {
    [&application.applicants[..], &application.auditors[..]].concat()
}

