// function getLocalStorage(key) {
//   chrome.storage.local.get([key], function(item) {
//     // console.log(item);
//     return item;
//   });
// }
//
document.getElementById("submit").addEventListener("click", submitted);

function submitted() {
  chrome.storage.local.get(["key"], function(item) {
    console.log(item);
  });
  var input = document.getElementById("key");
  var key = input.value;
  input.value = "";
  chrome.storage.local.set({ "key": key }, function() {
    console.log(key + " saved.");
  });
}
