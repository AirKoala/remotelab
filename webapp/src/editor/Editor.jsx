// import MonacoEditor from "@monaco-editor/react";
import ExampleProgram from "../assets/blink.ino?raw";
import { useState } from "react";

import AceEditor from "react-ace";
import "ace-builds/src-noconflict/mode-c_cpp";
import Console from "./Console";

import axios from "axios";

// import { ref, uploadString } from "firebase/storage";
// import firebaseApp from "../firebaseApp";

export default function Editor() {
  const [sourceFile, setSourceFile] = useState(ExampleProgram);
  const [consoleLines, setConsoleLines] = useState([
    "Hello World!",
  ]);

  // Hardcoded for now
  const projectId = 0; // TODO

  function upload(sourceCode) {
    console.log("Uploading...");
    const server = "http://10.42.0.129:3000";
    // fetch(server).then((response) => {
    //   console.log(response);
    // });
    // axios.get(server + "/").then((response) => {
    //   console.log(response);
    // }).catch((error) => {
    //   console.log(error);
    // });
    //

    setConsoleLines(["Uploading..."]);

    axios.post(server + "/upload-program", {
      "main.ino": sourceCode,
    }, {
      headers: {
        "Content-Type": "multipart/form-data",
        // "Access-Control-Allow-Origin": "*",
      },
    }).then((response) => {
      console.log(response.data);
      setConsoleLines(response.data.split(/\r?\n/));
    });
    // const fileRef = ref(firebaseApp.storage, "/projects/projectId/0.ino");
    // uploadString(fileRef, sourceCode, "raw", { contentType: "text/ino" })
    //   .then((snapshot) => {
    //     console.log("Upload complete!");
    //   })
    //   .catch((error) => {
    //     console.log("Upload failed!");
    //     console.log(error);
    //   });
  }

  return (
    <>
      <button onClick={() => upload(sourceFile)}>
        Upload
      </button>
      <AceEditor
        height="500px"
        width="100%"
        defaultValue={sourceFile}
        mode="c_cpp"
        onChange={(value) => {
          setSourceFile(value);
        }}
        // maxLines={Infinity}
        setOptions={{
          enableBasicAutocompletion: true,
          enableLiveAutocompletion: true,
          enableSnippets: true,
        }}
      />
      <Console
        lines={consoleLines}
      />
    </>
  );
}
