import { got_key } from "/brisk_auth_extension.js"

export function getLocalStorage(key) {
  // chrome.storage.local.get([key], function(item) {
  //   if (item === undefined) {
  //     return "0";
  //   } else {
  //     return item;
  //   }
  // });

  // var promise = { ran: false, result: null };
  // chrome.storage.local.get([key]).then((result) => {
  //   promise.result = result;
  //   promise.ran = true;
  // });
  // return promise;

  chrome.storage.local.get([key]).then((result) => {
    got_key(result.key);
  });
}


