#[cfg(test)]
mod tests {
    use task_drop_trait::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let _bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        // The bomb should not panic when defused
        bomb.defuse();
    }
}
