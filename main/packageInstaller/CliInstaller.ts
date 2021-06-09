import * as execa from 'execa';
import executeProfile from '../utils/executeProfile';
import { IPackageInfo, IPackageInstaller } from '../types';
import log from '../utils/log';
import ensureProfileExists from '../utils/ensureProfileExists';
import writeLog from '../utils/writeLog';

class CliInstaller implements IPackageInstaller {
  channel: string;

  cliProcessor: { [k: string]: Function };

  nodeProcessor: { [k: string]: Function };

  constructor(channel: string) {
    this.channel = channel;
    this.cliProcessor = {
      node: this.installNode,
    };

    this.nodeProcessor = {
      nvm: this.installNvm,
    };
  }

  install = async (packageInfo: IPackageInfo, shPath: string) => {
    const { name } = packageInfo;
    const installFunc = this.cliProcessor[name];
    if (installFunc) {
      await installFunc({ shPath, packageInfo });
    }
    // TODO: return node local path
    return { name, localPath: null };
  };

  private installNode = async ({ shPath, packageInfo }) => {
    const { options } = packageInfo;
    const { managerName } = options;
    const installManagerFunc = this.nodeProcessor[managerName];
    if (installManagerFunc) {
      await installManagerFunc({ shPath });
    }
  };

  private installNvm = ({ shPath }) => {
    ensureProfileExists();

    return new Promise((resolve, reject) => {
      let installStdout = '';
      const listenFunc = (buffer: Buffer) => {
        const chunk = buffer.toString();
        installStdout += chunk;
        process.send({ channel: this.channel, data: { chunk, ln: false } });
      };

      const cp = execa('sh', [shPath]);

      cp.stdout.on('data', listenFunc);

      cp.stderr.on('data', listenFunc);

      cp.on('error', (buffer: Buffer) => {
        const chunk = buffer.toString();
        writeLog(this.channel, chunk, true, 'error');
        reject(chunk);
      });

      cp.on('exit', () => {
        log.info(installStdout);
        const nvmProfileMatchRes = installStdout.match(/^(?:=> Appending nvm source string to|=> nvm source string already in) (.*)/);
        if (nvmProfileMatchRes) {
          const nvmProfilePath = nvmProfileMatchRes[1];
          // ensure nvm envs are in current shell environment
          executeProfile(nvmProfilePath);
        }
        resolve(null);
      });
    });
  };
}

export default CliInstaller;
