import { Table, Button, Message, Icon, Dropdown, Grid } from '@alifd/next';
import BalloonConfirm from '@/components/BalloonConfirm';
import CustomIcon from '@/components/Icon';
import { useEffect } from 'react';
import { INpmDependency } from '@/interfaces/npmDependency';
import store from '../../store';
import InstallNpmDependency from '../InstallNpmDependency';
import styles from './index.module.scss';

const { Row, Col } = Grid;

function NpmDependency() {
  const [state, dispatcher] = store.useModel('npmDependency');
  const effectsState = store.useModelEffectsState('npmDependency');
  const { npmDependencies, curUpdateDepIndex, curUninstallDepIndex, curReinstallDepIndex } = state;

  useEffect(() => {
    if (effectsState.getGlobalNpmDependencies.error) {
      Message.error(effectsState.getGlobalNpmDependencies.error.message);
    }
  }, [effectsState.getGlobalNpmDependencies.error]);

  useEffect(() => {
    if (effectsState.uninstallGlobalNpmDependency.error) {
      Message.error(effectsState.uninstallGlobalNpmDependency.error.message);
    }
  }, [effectsState.uninstallGlobalNpmDependency.error]);

  useEffect(() => {
    if (effectsState.updateGlobalNpmDependency.error) {
      Message.error(effectsState.updateGlobalNpmDependency.error.message);
    }
  }, [effectsState.updateGlobalNpmDependency.error]);

  useEffect(() => {
    if (effectsState.reinstallGlobalNpmDependency.error) {
      Message.error(effectsState.reinstallGlobalNpmDependency.error.message);
    }
  }, [effectsState.reinstallGlobalNpmDependency.error]);

  const onUninstallGlobalDep = async (dependency: INpmDependency, index: number) => {
    dispatcher.addCurDepIndex({ type: 'uninstall', index });
    const { name } = dependency;
    await dispatcher.uninstallGlobalNpmDependency(name);
    dispatcher.removeCurDepIndex({ type: 'uninstall', index });
    Message.success(`卸载依赖 ${name} 成功`);
    await dispatcher.getGlobalNpmDependencies(true);
  };

  const onUpdateGlobalDep = async (dependency: INpmDependency, index: number) => {
    dispatcher.addCurDepIndex({ type: 'update', index });
    const { name } = dependency;
    await dispatcher.updateGlobalNpmDependency(dependency.name);
    dispatcher.removeCurDepIndex({ type: 'update', index });
    Message.success(`升级依赖 ${name} 成功`);
    await dispatcher.getGlobalNpmDependencies(true);
  };

  const onReinstallGlobalDep = async (dependency: INpmDependency, index: number) => {
    dispatcher.addCurDepIndex({ type: 'reinstall', index });
    const { name, currentVersion } = dependency;
    await dispatcher.reinstallGlobalNpmDependency({ dependency: name, version: currentVersion });
    dispatcher.removeCurDepIndex({ type: 'reinstall', index });
    Message.success(`重装依赖 ${name}@${currentVersion} 成功`);
    await dispatcher.getGlobalNpmDependencies(true);
  };

  const operationRender = (value: any, index: number, record: INpmDependency) => {
    const isReinstallCurrentDep = curReinstallDepIndex.includes(index) && effectsState.reinstallGlobalNpmDependency.isLoading;
    const isUninstallCurrentDep = curUninstallDepIndex.includes(index) && effectsState.uninstallGlobalNpmDependency.isLoading;

    return (
      <div className={styles.columnCell}>
        <BalloonConfirm
          onConfirm={async () => await onReinstallGlobalDep(record, index)}
          title="确定重装该依赖？"
          disable={isReinstallCurrentDep}
        >
          <Button
            text
            type="primary"
            disabled={isReinstallCurrentDep}
          >
            {isReinstallCurrentDep ? <Icon type="loading" /> : <CustomIcon type="gongju" />}
          </Button>
        </BalloonConfirm>
        <BalloonConfirm
          onConfirm={async () => await onUninstallGlobalDep(record, index)}
          title="确定卸载该依赖？"
          disable={isUninstallCurrentDep}
        >
          <Button
            className={styles.button}
            text
            type="primary"
            disabled={isUninstallCurrentDep}
          >
            {isUninstallCurrentDep ? <Icon type="loading" /> : <CustomIcon type="trash" />}
          </Button>
        </BalloonConfirm>
      </div>
    );
  };

  const latestVersionRender = (value: string, index: number, record: INpmDependency) => {
    const isUpdateGlobalDep = curUpdateDepIndex.includes(index) && effectsState.updateGlobalNpmDependency.isLoading;

    return (
      <div className={styles.columnCell}>
        <span>{value}</span>
        {value && (
        <Button
          className={styles.button}
          text
          type="primary"
          onClick={async () => await onUpdateGlobalDep(record, index)}
          disabled={isUpdateGlobalDep}
        >
          {isUpdateGlobalDep ? <Icon type="loading" /> : <CustomIcon type="jiantouarrow499" />}
        </Button>
        )}
      </div>
    );
  };

  useEffect(() => {
    dispatcher.getGlobalNpmDependencies();
  }, []);
  return (
    <>
      <Row className={styles.header}>
        <div className={styles.title}>全局 npm 依赖管理</div>
        <Dropdown trigger={<Button type="primary">添加依赖</Button>} triggerType={['click']}>
          <InstallNpmDependency />
        </Dropdown>
      </Row>
      <Table loading={effectsState.getGlobalNpmDependencies.isLoading} dataSource={npmDependencies} className={styles.table}>
        <Table.Column title="npm 依赖" dataIndex="name" width={200} />
        <Table.Column title="当前版本" dataIndex="currentVersion" width={200} />
        <Table.Column title="最新版本" dataIndex="latestVersion" cell={latestVersionRender} width={200} />
        <Table.Column title="操作" cell={operationRender} width={200} />
      </Table>
    </>
  );
}

export default NpmDependency;
