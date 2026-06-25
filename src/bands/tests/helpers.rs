    fn test_tab_root(name: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("coronatio-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        root
    }
