// Client
const folio = "FOL-002";
const titulo = "Credencial perdida";

const [reportePda] = web3.PublicKey.findProgramAddressSync(
  [
      Buffer.from("reporte"),
          pg.wallet.publicKey.toBuffer(),
              Buffer.from(folio),
                ],
                  pg.program.programId
                  );

                  console.log("PDA calculada:", reportePda.toBase58());

                  const tx = await pg.program.methods
                    .crearReporte(folio, titulo)
                      .accounts({
                          reporte: reportePda,
                              autor: pg.wallet.publicKey,
                                  systemProgram: web3.SystemProgram.programId,
                                    })
                                      .rpc();

                                      console.log("Reporte creado! Tx:", tx);
                                      