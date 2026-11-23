// Client
const reportePda = new web3.PublicKey("Ghp75Tdx78kfkos9h2ri4UZ3LHN8rWSPj9zWJYRTsknn");

const reporteData = await pg.program.account.reporte.fetch(reportePda);
console.log("Datos del reporte en Devnet:", reporteData);
