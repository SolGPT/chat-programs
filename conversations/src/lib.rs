mod instruction;
mod state;
mod processor;

// Import items from other modules to use them in this file
use instruction::{process_instruction};

use solana_program::{
    entrypoint,
};

entrypoint!(process_instruction);
