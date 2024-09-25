import { useState, useEffect } from 'react';
import { hello_backend } from 'declarations/hello_backend';

function App() {
  const [heartbeats, setHeartbeats] = useState([]);
  const [subject, setAppId] = useState('');
  const [time, setStatus] = useState('');

  useEffect(() => {
    fetchHeartbeats();
  }, []);

  async function fetchHeartbeats() {
    const result = await hello_backend.get_all_heartbeats();
    setHeartbeats(result);
  }

  async function handleSubmit(event) {
    event.preventDefault();
    if (subject && time) {
      await hello_backend.record_heartbeat(subject, time);
      setAppId('');
      setStatus('');
      fetchHeartbeats();
    }
  }

  return (
    <main>
      <img src="/logo2.svg" alt="DFINITY logo" />
      <br />
      <br />
      <form onSubmit={handleSubmit}>
        <label htmlFor="appId">Subject: &nbsp;</label>
        <input
          id="appId"
          type="text"
          value={subject}
          onChange={(e) => setAppId(e.target.value)}
          required
        />
        <label htmlFor="status">&nbsp; Time: &nbsp;</label>
        <input
          id="status"
          type="text"
          value={time}
          onChange={(e) => setStatus(e.target.value)}
          required
        />
        <button type="submit">Record Heartbeat</button>
      </form>
      <section id="heartbeats">
        <h2>Latest Heartbeats</h2>
        <ul>
          {heartbeats.map(([app, status], index) => (
            <li key={index}>
              {app}: {status}
            </li>
          ))}
        </ul>
      </section>
    </main>
  );
}

export default App;