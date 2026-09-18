use widya::bootstrap::{SelfHostingBootstrap, BootstrapStageStatus};

#[test]
fn test_self_hosted_modules_verification() {
    let bootstrapper = SelfHostingBootstrap::new("compiler_self_hosted");
    
    // Verifikasi masing-masing modul
    let token_res = bootstrapper.verify_module("token.widya");
    assert!(token_res.is_ok(), "Gagal verifikasi token.widya: {:?}", token_res.err());

    let lexer_res = bootstrapper.verify_module("lexer.widya");
    assert!(lexer_res.is_ok(), "Gagal verifikasi lexer.widya: {:?}", lexer_res.err());

    let ast_res = bootstrapper.verify_module("ast.widya");
    assert!(ast_res.is_ok(), "Gagal verifikasi ast.widya: {:?}", ast_res.err());

    let parser_res = bootstrapper.verify_module("parser.widya");
    assert!(parser_res.is_ok(), "Gagal verifikasi parser.widya: {:?}", parser_res.err());

    let typesystem_res = bootstrapper.verify_module("typesystem.widya");
    assert!(typesystem_res.is_ok(), "Gagal verifikasi typesystem.widya: {:?}", typesystem_res.err());

    let codegen_res = bootstrapper.verify_module("codegen.widya");
    assert!(codegen_res.is_ok(), "Gagal verifikasi codegen.widya: {:?}", codegen_res.err());

    let main_res = bootstrapper.verify_module("main.widya");
    assert!(main_res.is_ok(), "Gagal verifikasi main.widya: {:?}", main_res.err());
}

#[test]
fn test_full_bootstrap_stages_cycle() {
    let bootstrapper = SelfHostingBootstrap::new("compiler_self_hosted");
    let stages = bootstrapper.run_full_bootstrap_cycle();
    
    assert_eq!(stages.len(), 3, "Harus menyelesaikan Stage-0, Stage-1, dan Stage-2");
    for stage in stages {
        match stage {
            BootstrapStageStatus::Success { stage_name, hash } => {
                assert!(!hash.is_empty(), "Hash tahapan {} tidak boleh kosong", stage_name);
            }
            BootstrapStageStatus::Failure { stage_name, error_msg } => {
                panic!("Tahapan {} gagal: {}", stage_name, error_msg);
            }
        }
    }
}
