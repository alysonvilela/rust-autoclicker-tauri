import { useEffect, useState } from "react";
// When using the Tauri API npm package:
import { invoke } from "@tauri-apps/api/tauri";

import "./App.css";
import { emit, listen } from "@tauri-apps/api/event";

interface Position {
  start: {
    loading: boolean;
    pos: [null | number, null | number];
  };
  end: {
    loading: boolean;
    pos: [null | number, null | number];
  };
}

const App = () => {
  const [_appName, setApp] = useState<null | string>(null);
  const [position, setPosition] = useState<Position>({
    start: {
      loading: false,
      pos: [null, null],
    },
    end: {
      loading: false,
      pos: [null, null],
    },
  });

  const validPosition = !!position.start.pos[0] && !!position.end.pos[0];

  const onRecordStart = async () => {
    await invoke("get_next_click", {
      status: "start",
    });
    setPosition((prev) => ({
      ...prev,
      start: {
        loading: true,
        pos: [...prev.start.pos],
      },
    }));
  };

  const onRecordEnd = async () => {
    await invoke("get_next_click", {
      status: "end",
    });
    setPosition((prev) => ({
      ...prev,
      end: {
        loading: true,
        pos: [...prev.end.pos],
      },
    }));
  };

  const onInitAfk = async () => {
    emit("run-move-mouse", {
      startPos: position.start.pos,
      endPos: position.end.pos,
    });
  };

  useEffect(() => {
    (async () => {
      await listen("start_position_at", (ev) =>
        setPosition((prev) => ({
          ...prev,
          start: {
            loading: false,
            pos: ev.payload as [number, number],
          },
        }))
      );
    })();
  }, [position]);

  useEffect(() => {
    (async () => {
      await listen("end_position_at", (ev) =>
        setPosition((prev) => ({
          ...prev,
          end: {
            loading: false,
            pos: ev.payload as [number, number],
          },
        }))
      );
    })();
  }, [position]);

  return (
    <div className="container">
      <h1>Auto clicker</h1>

      {/* <p>Click on the Tauri, Vite, and React logos to learn more.</p> */}

      <div className="form">
        <div className="inputWrapper">
          <label htmlFor="app"> App name</label>
          <input
            id="app"
            onChange={(e) => setApp(e.currentTarget.value)}
            placeholder="App name ex: 'Slack'"
          />
        </div>

        <div className="inputWrapper">
          <label htmlFor="p1">Start position</label>
          <button onClick={onRecordStart}>
            {position.start.loading
              ? "..."
              : `${!!position.start.pos[0] ? `(${position.start.pos[0]}, ${position.start.pos[1]} - Change)` : "Record"}`}
          </button>
        </div>
        <div className="inputWrapper">
          <label htmlFor="p2">End position</label>
          <button onClick={onRecordEnd}>
            {position.end.loading
              ? "..."
              : `${!!position.end.pos[0] ? `(${position.end.pos[0]}, ${position.end.pos[1]} - Change)` : "Record"}`}
          </button>
        </div>
      </div>
      {/* <pre>{JSON.stringify(position, null, 2)}</pre> */}
      <button
        // disabled={!validPosition}
        className="primary"
        type="button"
        onClick={onInitAfk}
      >
        Start
      </button>
    </div>
  );
};

export default App;
