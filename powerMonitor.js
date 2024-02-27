"use strict";

const electron = require('electron');
const {
  POWER_MONITOR_GET_SYSTEM_IDLE_TIME
} = require('../common/constants').IPCEvents;

electron.ipcMain.removeHandler(POWER_MONITOR_GET_SYSTEM_IDLE_TIME);

electron.powerMonitor.removeAllListeners('resume');
electron.powerMonitor.removeAllListeners('suspend');
electron.powerMonitor.removeAllListeners('lock-screen');
electron.powerMonitor.removeAllListeners('unlock-screen'); 
