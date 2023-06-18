pub struct CloseHandle {
    is_close:  bool
}

impl CloseHandle {
    pub fn new() -> CloseHandle {
        CloseHandle {
            is_close: false
        }
    }

    pub fn is_closed(&self) -> bool {
        return self.is_close
    }

    pub fn close(&mut self) {
        self.is_close = true;
    }
}