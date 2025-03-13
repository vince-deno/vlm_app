pub trait StyleConfigurable {
    /// Apply a single style (e.g., "red", "underline", "mark") to the given text.
    fn apply_style(&self, style: &str, text: &str) -> String;

    /// Apply a composite style which is a colon-separated string (e.g., "underline:red").
    /// This method splits the composite string and chains the style effects.
    fn apply_composite_style(&self, composite: &str, text: &str) -> String {
        // Split by colon. Expecting formats like "underline:red" or "mark:blue"
        let parts: Vec<&str> = composite.split(':').collect();
        if parts.len() == 1 {
            // If there is only one part, apply that style directly.
            self.apply_style(parts[0], text)
        } else if parts.len() == 2 {
            // Apply the parameter style first (for color/mark), then the base style (e.g., underline)
            let base = parts[0];
            let param = parts[1];
            // First, apply the parameter style
            let intermediate = self.apply_style(param, text);
            // Then, apply the base style on top
            self.apply_style(base, &intermediate)
        } else {
            // If the format is unexpected, return the text unmodified.
            text.to_string()
        }
    }
}
