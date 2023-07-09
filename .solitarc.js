const path = require("path");
const programDir = path.join(__dirname, ".", "programs/derug-program");
const idlDir = path.join(__dirname, "target/idl");
const sdkDir = path.join(__dirname, "src", "generated");
const binaryInstallDir = path.join(__dirname, ".crates");

module.exports = {
  idlGenerator: "anchor",
  programName: "derug_program",
  programId: "DERUGwXJu3m1DG1VNq4gP7Ppkza95P7XbeujbtSNAebu",
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
