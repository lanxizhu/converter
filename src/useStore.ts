import { load } from '@tauri-apps/plugin-store';
import { FileInfo } from './main';

export const HISTORY_FILES_KEY = 'history-files';

// Create a new store or load the existing one,
// note that the options will be ignored if a `Store` with that path has already been created
export const store = await load('store.json', { autoSave: true });

// Set a value.
await store.set('some-key', { value: 5 });

// await store.set('history-files', []);

// Get a value.
const val = await store.get<{ value: number }>('some-key');
console.log(val); // { value: 5 }

// You can manually save the store after making changes.
// Otherwise, it will save upon graceful exit
// And if you set `autoSave` to a number or left empty,
// it will save the changes to disk after a debounce delay, 100ms by default.
await store.save();

const historyFiles = await store.get<FileInfo[]>(HISTORY_FILES_KEY);
if (!historyFiles || !Array.isArray(historyFiles)) {
  await store.set(HISTORY_FILES_KEY, []);
}
