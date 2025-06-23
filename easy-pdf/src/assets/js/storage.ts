function saveData(key: string, value: any) {
  const dbRequest = indexedDB.open('myDb', 1);

  dbRequest.onupgradeneeded = () => {
    const db = dbRequest.result;
    db.createObjectStore('storage', { keyPath: 'key' });
  };

  dbRequest.onsuccess = () => {
    const db = dbRequest.result;
    const transaction = db.transaction('storage', 'readwrite');
    const store = transaction.objectStore('storage');
    store.put({ key, value });
  };
}

function getData(key: string) {
  return new Promise((resolve, reject) => {
    const dbRequest = indexedDB.open('myDb', 1);

    dbRequest.onsuccess = () => {
      const db = dbRequest.result;
      const transaction = db.transaction('storage', 'readonly');
      const store = transaction.objectStore('storage');
      const request = store.get(key);

      request.onsuccess = () => {
        resolve(request.result ? request.result.value : null);
      };

      request.onerror = (err) => {
        reject(err);
      };
    };
  });
}



export { saveData, getData };