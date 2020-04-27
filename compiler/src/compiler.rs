use crate::{ast, Program, ResolvedProgram};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::{ConstraintSynthesizer, ConstraintSystem},
};

use from_pest::FromPest;
use std::{
    fs,
    marker::PhantomData,
    path::PathBuf,
};

#[derive(Clone)]
pub struct Compiler<F: Field + PrimeField> {
    main_file_path: PathBuf,
    _engine: PhantomData<F>,
}

impl<F: Field + PrimeField> Compiler<F> {
    pub fn init(main_file_path: PathBuf) -> Self {
        Self { main_file_path, _engine: PhantomData }
    }
}

impl<F: Field + PrimeField> ConstraintSynthesizer<F> for Compiler<F> {
    fn generate_constraints<CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        // Read in the main file as string
        let unparsed_file = fs::read_to_string(&self.main_file_path).expect("cannot read file");

        // Parse the file using leo.pest
        let mut file = ast::parse(&unparsed_file).expect("unsuccessful parse");

        // Build the abstract syntax tree
        let syntax_tree = ast::File::from_pest(&mut file).expect("infallible");
        log::debug!("{:#?}", syntax_tree);

        let program = Program::<'_, F>::from(syntax_tree);
        log::info!(" compiled: {:#?}", program);

        let program = program.name("simple".into());
        ResolvedProgram::generate_constraints(cs, program);

        Ok(())
    }
}