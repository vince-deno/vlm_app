
#[derive(Debug, Clone)]
pub enum Scope {
    Admin,
    Developer,
    User,
}

#[derive(Clone)]
pub enum VLMScopeSpec<'a> {
    Public(i32),
    Private(&'a dyn Fn(),i64),
    Count(isize),
}



impl From<String> for Scope {
    fn from(scope: String) -> Self {
        match scope.as_str() {
            "admin" => Scope::Admin,
            "developer" => Scope::Developer,
            "user" => Scope::User,
            _ => Scope::User, // Default case if scope is invalid
        }
    }
}

impl Scope {
    // A method to simulate checking the current scope
    pub fn current() -> Self {
        Scope::Developer // Example, assuming the current scope is Developer
    }

    // Validation function for checking the scope
    pub fn validate(&self, required: &Scope) -> Result<(), String> {
        if *self >= *required {
            Ok(())
        } else {
            Err(format!(
                "Permission denied: Current scope ({:?}) does not meet required scope ({:?}).",
                self, required
            ))
        }
    }
}

// For comparison in validation (if needed)
impl Eq for Scope {}

// impliment vlm scope spec

impl<'a> PartialEq for VLMScopeSpec<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Public(l0), Self::Public(r0)) => l0 == r0,
            (Self::Private(l0, l1), Self::Private(r0, r1)) => std::ptr::eq(*l0 as *const _, *r0 as *const _) && l1 == r1,
            (Self::Count(l0), Self::Count(r0)) => l0 == r0,
            _ => false,
        }
    }
    
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}


impl<'a> PartialOrd for VLMScopeSpec<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Public(l0), Self::Public(r0)) => l0.partial_cmp(r0),
            (Self::Private(_, l1), Self::Private(_, r1)) => l1.partial_cmp(r1),
            (Self::Count(l0), Self::Count(r0)) => l0.partial_cmp(r0),
            _ => None,
        }
    }
    
    fn lt(&self, other: &Self) -> bool {
        std::matches!(self.partial_cmp(other), Some(std::cmp::Ordering::Less))
    }
    
    fn le(&self, other: &Self) -> bool {
        std::matches!(self.partial_cmp(other), Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal))
    }
    
    fn gt(&self, other: &Self) -> bool {
        std::matches!(self.partial_cmp(other), Some(std::cmp::Ordering::Greater))
    }
    
    fn ge(&self, other: &Self) -> bool {
        std::matches!(self.partial_cmp(other), Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal))
    }
}

impl PartialEq for Scope {
    fn eq(&self, other: &Self) -> bool {
        use Scope::*;
        matches!((self, other), 
            (Admin, Admin) | 
            (Developer, Developer) | 
            (User, User)
        )
    }
}

impl PartialOrd for Scope {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Scope::*;
        Some(match (self, other) {
            (Admin, Admin) | (Developer, Developer) | (User, User) => std::cmp::Ordering::Equal,
            (Admin, _) => std::cmp::Ordering::Greater,
            (_, Admin) => std::cmp::Ordering::Less,
            (Developer, _) => std::cmp::Ordering::Greater,
            (_, Developer) => std::cmp::Ordering::Less,
        })
    }
    
}

impl Ord for Scope {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
    
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }
    
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }
    
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}
