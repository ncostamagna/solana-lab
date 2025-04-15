use anchor_lang::prelude::*;

declare_id!("EN8B9opTVhshRSvENhJAsPUqtEuqZfWaBRQTfjoFtZho");

#[program]
pub mod mi_primer_proyecto {
    use super::*;

    pub fn inicializar(ctx: Context<Inicializar>) -> Result<()> {
        let contador = &mut ctx.accounts.contador;
        contador.valor = 0;
        msg!("¡Contador inicializado en 0!");
        Ok(())
    }

    pub fn incrementar(ctx: Context<Actualizar>) -> Result<()> {
        let contador = &mut ctx.accounts.contador;
        contador.valor += 1;
        msg!("¡Contador incrementado! Nuevo valor: {}", contador.valor);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Inicializar<'info> {
    #[account(init, payer = usuario, space = 8 + 8)]
    pub contador: Account<'info, Contador>,
    #[account(mut)]
    pub usuario: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Actualizar<'info> {
    #[account(mut)]
    pub contador: Account<'info, Contador>,
}

#[account]
pub struct Contador {
    pub valor: u64,
}