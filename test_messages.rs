use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::MessageTemplates;

    #[test]
    fn test_harvest_message_variations() {
        let mut messages = MessageTemplates::new(Some(42));
        let mut variations = HashSet::new();
        
        // Generate 50 messages to ensure we get variations
        for _ in 0..50 {
            let msg = messages.harvest_message(3, 300);
            variations.insert(msg);
        }
        
        // Should have multiple variations
        assert!(variations.len() > 1, "Expected multiple harvest message variations, got {}", variations.len());
        
        // All messages should contain the correct values
        for msg in &variations {
            assert!(msg.contains("3"), "Message should contain yield per acre: {}", msg);
            assert!(msg.contains("300"), "Message should contain total harvest: {}", msg);
        }
    }

    #[test]
    fn test_all_message_types_have_variations() {
        let mut messages = MessageTemplates::new(Some(42));
        
        // Test each message type generates variations
        let mut harvest_variations = HashSet::new();
        let mut rats_variations = HashSet::new();
        let mut plague_variations = HashSet::new();
        let mut immigration_variations = HashSet::new();
        let mut no_immigration_variations = HashSet::new();
        let mut starvation_variations = HashSet::new();
        
        for _ in 0..50 {
            harvest_variations.insert(messages.harvest_message(3, 300));
            rats_variations.insert(messages.rats_message(100));
            plague_variations.insert(messages.plague_message());
            immigration_variations.insert(messages.immigration_message(20));
            no_immigration_variations.insert(messages.no_immigration_message());
            starvation_variations.insert(messages.starvation_message(5));
        }
        
        assert!(harvest_variations.len() > 1, "Harvest messages lack variation");
        assert!(rats_variations.len() > 1, "Rats messages lack variation");
        assert!(plague_variations.len() > 1, "Plague messages lack variation");
        assert!(immigration_variations.len() > 1, "Immigration messages lack variation");
        assert!(no_immigration_variations.len() > 1, "No immigration messages lack variation");
        assert!(starvation_variations.len() > 1, "Starvation messages lack variation");
    }

    #[test]
    fn test_insufficient_messages_have_variations() {
        let mut messages = MessageTemplates::new(Some(42));
        
        let mut grain_land_variations = HashSet::new();
        let mut land_variations = HashSet::new();
        let mut grain_seed_variations = HashSet::new();
        let mut land_planting_variations = HashSet::new();
        let mut workers_variations = HashSet::new();
        let mut grain_feeding_variations = HashSet::new();
        
        for _ in 0..50 {
            grain_land_variations.insert(messages.insufficient_grain_land_message());
            land_variations.insert(messages.insufficient_land_message());
            grain_seed_variations.insert(messages.insufficient_grain_seed_message());
            land_planting_variations.insert(messages.insufficient_land_planting_message());
            workers_variations.insert(messages.insufficient_workers_message());
            grain_feeding_variations.insert(messages.insufficient_grain_feeding_message());
        }
        
        assert!(grain_land_variations.len() > 1, "Insufficient grain (land) messages lack variation");
        assert!(land_variations.len() > 1, "Insufficient land messages lack variation");
        assert!(grain_seed_variations.len() > 1, "Insufficient grain (seed) messages lack variation");
        assert!(land_planting_variations.len() > 1, "Insufficient land (planting) messages lack variation");
        assert!(workers_variations.len() > 1, "Insufficient workers messages lack variation");
        assert!(grain_feeding_variations.len() > 1, "Insufficient grain (feeding) messages lack variation");
    }

    #[test]
    fn test_message_formatting() {
        let mut messages = MessageTemplates::new(Some(42));
        
        // Test harvest message formatting
        let harvest_msg = messages.harvest_message(5, 500);
        assert!(!harvest_msg.contains("{}"), "Harvest message contains unformatted placeholder");
        
        // Test rats message formatting
        let rats_msg = messages.rats_message(123);
        assert!(!rats_msg.contains("{}"), "Rats message contains unformatted placeholder");
        assert!(rats_msg.contains("123"), "Rats message missing value");
        
        // Test immigration message formatting
        let immigration_msg = messages.immigration_message(42);
        assert!(!immigration_msg.contains("{}"), "Immigration message contains unformatted placeholder");
        assert!(immigration_msg.contains("42"), "Immigration message missing value");
        
        // Test starvation message formatting
        let starvation_msg = messages.starvation_message(7);
        assert!(!starvation_msg.contains("{}"), "Starvation message contains unformatted placeholder");
        assert!(starvation_msg.contains("7"), "Starvation message missing value");
    }
}