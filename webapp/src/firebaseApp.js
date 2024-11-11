import { initializeApp } from "firebase/app";
import { getStorage, connectStorageEmulator } from "firebase/storage";
import config from "./firebaseConfig";

// Initialize Firebase
const app = initializeApp(config);

// Firebase storage reference
const storage = getStorage(app);

// Connect to the Storage emulator running locally.
if (location.hostname === "localhost") {
  // Point to the Storage emulator running on localhost.
  connectStorageEmulator(storage, "127.0.0.1", 9199);
}


export default {
    storage,
    app
}