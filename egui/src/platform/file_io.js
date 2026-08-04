// Browser side of the app's file handling (see platform/web.rs).
//
// Two capability levels, picked at runtime so the app works on a plain
// http:// intranet host as well as over https://:
//
//   * File System Access API (Chrome/Edge, and only in a secure context):
//     real Open/Save dialogs, and "Save" writes straight back to the file the
//     plan was opened from — same behaviour as the desktop build.
//   * Everywhere else (Firefox, Safari, any plain-http host): an <input
//     type="file"> to read, and a download to write.  The browser decides
//     where the file lands, so "Save" and "Save As..." both behave like
//     "Save As...".
//
// Nothing is uploaded anywhere: the plan is read into the page, edited in
// memory, and written back out only when the user asks.

// The file the plan was opened from / last saved to, when the browser lets us
// hold on to it.  Used so "Save" can overwrite in place.
let currentFile = null;

const FILE_TYPES = [
  {
    description: "Financial plan",
    accept: { "application/json": [".json"] },
  },
];

const hasFileSystemAccess =
  typeof window !== "undefined" &&
  typeof window.showOpenFilePicker === "function" &&
  typeof window.showSaveFilePicker === "function";

/// Ask the user for a plan file.
///
/// Resolves to `{ name, text }`, or `null` if the user cancelled.
export async function openJsonFile() {
  if (hasFileSystemAccess) {
    let handles;
    try {
      handles = await window.showOpenFilePicker({
        multiple: false,
        types: FILE_TYPES,
        excludeAcceptAllOption: false,
      });
    } catch (err) {
      if (isCancellation(err)) return null;
      throw err;
    }
    const handle = handles[0];
    const file = await handle.getFile();
    const text = await file.text();
    // Remember it so "Save" can write back to this very file
    currentFile = handle;
    return { name: file.name, text };
  }

  const file = await pickViaInput();
  if (file === null) return null;
  currentFile = null;
  return { name: file.name, text: await file.text() };
}

/// Write `text` back out to the user's machine.
///
/// With `inPlace` set, overwrite the file the plan came from if we still have a
/// writable handle for it; otherwise ask where to put it.  Resolves to
/// `{ name }`, or `null` if the user cancelled.
export async function saveJsonFile(suggestedName, text, inPlace) {
  if (hasFileSystemAccess) {
    let handle = inPlace ? await writableHandle(currentFile) : null;
    if (handle === null) {
      try {
        handle = await window.showSaveFilePicker({
          suggestedName,
          types: FILE_TYPES,
        });
      } catch (err) {
        if (isCancellation(err)) return null;
        throw err;
      }
    }
    const writable = await handle.createWritable();
    await writable.write(text);
    await writable.close();
    currentFile = handle;
    return { name: handle.name };
  }

  downloadFile(suggestedName, text);
  return { name: suggestedName };
}

/// Open a throwaway file input and resolve with the chosen `File`, or `null`.
function pickViaInput() {
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = ".json,application/json";
    input.style.display = "none";
    document.body.appendChild(input);

    const done = (file) => {
      input.remove();
      resolve(file);
    };
    input.addEventListener("change", () =>
      done(input.files && input.files.length > 0 ? input.files[0] : null),
    );
    // Fires when the picker is dismissed without a selection
    input.addEventListener("cancel", () => done(null));
    input.click();
  });
}

/// Hand the bytes to the browser as a download.
function downloadFile(name, text) {
  const url = URL.createObjectURL(
    new Blob([text], { type: "application/json" }),
  );
  const link = document.createElement("a");
  link.href = url;
  link.download = name;
  link.style.display = "none";
  document.body.appendChild(link);
  link.click();
  link.remove();
  // The download reads from the blob after click() returns, so keep the URL
  // alive for a while before letting the memory go
  setTimeout(() => URL.revokeObjectURL(url), 60_000);
}

/// The handle, if it is still writable — permission can lapse between visits,
/// and re-requesting it needs the user's say-so.  `null` means "ask again".
async function writableHandle(handle) {
  if (handle === null) return null;
  if (typeof handle.queryPermission !== "function") return handle;
  const options = { mode: "readwrite" };
  if ((await handle.queryPermission(options)) === "granted") return handle;
  try {
    if ((await handle.requestPermission(options)) === "granted") return handle;
  } catch (err) {
    if (!isCancellation(err)) throw err;
  }
  return null;
}

/// Dismissing a picker rejects with AbortError; that is a cancel, not a failure.
function isCancellation(err) {
  return err instanceof DOMException && err.name === "AbortError";
}
