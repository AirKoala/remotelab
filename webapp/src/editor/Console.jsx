export default function Console({ lines }) {
  return (
    <div className="console" style={{
      margin: 10,
      padding: 10,
      overflow: 'auto',
      backgroundColor: 'gray',
      color: '#fff',
      fontFamily: 'monospace',
      fontSize: 12,
    }}>
      {lines.map((line, index) => (
        <p key={index}>{line}</p>
      ))}
    </div>
  );
}