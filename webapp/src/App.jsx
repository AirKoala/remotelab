import Editor from './editor/Editor.jsx';
import { ErrorBoundary } from "react-error-boundary";

function App() {
  return (
    <ErrorBoundary fallback={<div>Something went wrong...</div>}>
      <Editor />
    </ErrorBoundary>
  );
}

export default App
