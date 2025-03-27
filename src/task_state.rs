#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug)]
pub enum ThreadStateEnum {
    ThreadStateInactive = 0,
    ThreadStateRunning = 1,
    ThreadStateRestart = 2,
    ThreadStateBlockedOnReceive = 3,
    ThreadStateBlockedOnSend = 4,
    ThreadStateBlockedOnReply = 5,
    ThreadStateBlockedOnNotification = 6,
    ThreadStateIdleThreadState = 7,
}