const path = require('path');
const programDir = path.join(__dirname, '..', 'programs/lollys-lotto');
const idlDir = path.join(__dirname, 'idl');
const sdkDir = path.join(__dirname, 'src');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
  idlGenerator: 'anchor',
  programName: 'lollys-lotto',
  programId: 'EQHT3TFXS3hBMzSpJiKb84sHE7iBXnYBpWvQTU8r91m6',
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
