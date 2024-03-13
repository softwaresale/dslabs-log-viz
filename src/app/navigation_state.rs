
#[derive(Debug, Default)]
pub struct NavigationState {
    /// the order of elements we should navigate to
    nav_order: Vec<usize>,
    /// the index in the nav order we are currently looking at
    current_idx: Option<usize>,
}

impl NavigationState {
    pub fn load_nav_order(&mut self, order: Vec<usize>) {
        self.nav_order = order;
        self.current_idx = None;
    }
    
    pub fn clear_nav_order(&mut self) {
        self.nav_order.clear();
        self.current_idx = None;
    }
    
    pub fn next_event(&mut self) -> Option<usize> {
        let next_idx = match self.current_idx {
            None => 0usize,
            Some(existing) => existing + 1,
        };
        
        match self.nav_order.get(next_idx) {
            None => None,
            Some(nav_order) => {
                self.current_idx = Some(next_idx);
                Some(*nav_order)
            }
        }
    }
    
    pub fn prev_event(&mut self) -> Option<usize> {
        let Some(existing) = self.current_idx else {
            return None
        };

        // if at zero, clear
        let prev_idx = if existing == 0 {
            self.current_idx = None;
            return None;
        } else {
            existing - 1
        };
        
        // otherwise, get
        match self.nav_order.get(prev_idx) {
            None => None,
            Some(nav_order) => {
                self.current_idx = Some(prev_idx);
                Some(*nav_order)
            }
        }
    }
}
