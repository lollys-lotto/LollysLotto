const path = require('path');
const programDir = path.join(__dirname, '..', 'programs/lollys-lotto');
const idlDir = path.join(__dirname, 'idl');
const sdkDir = path.join(__dirname, 'src');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
  idlGenerator: 'anchor',
  programName: 'lollys-lotto',
  programId: '13j1mUzsVqhSEiV3QP2oF2a4AUEQuhmQcjdF8nBrG1o1',
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
