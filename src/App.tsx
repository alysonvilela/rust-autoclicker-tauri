import { useEffect, useState } from "react";
// When using the Tauri API npm package:
import { invoke } from "@tauri-apps/api/tauri";
import { ChevronRight, PlayIcon } from "lucide-react";
import { emit, listen } from "@tauri-apps/api/event";

interface IPosition {
  start: {
    loading: boolean;
    pos: [null | number, null | number];
  };
  end: {
    loading: boolean;
    pos: [null | number, null | number];
  };
}

interface PositionProps {
  axis: IPosition["start"]["pos"];
}

export const LoadingSpinner = () => {
  return <i className="gg-spinner"></i>;
};

export const Position = ({ axis }: PositionProps) => {
  return (
    <div
      style={{
        fontSize: 12,
        display: "flex",
        alignItems: "center",
        gap: 4,
      }}
    >
      {axis[0]}, {axis[1]} <ChevronRight width={14} height={14} />
    </div>
  );
};

export const RecordLabel = () => {
  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        fontSize: 12,
        gap: 4,
      }}
    >
      Record <ChevronRight width={14} height={14} />
    </div>
  );
};

const App = () => {
  const [position, setPosition] = useState<IPosition>({
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
      <header>Auto clicker</header>
      <div className="form">
        <button className="row-button" onClick={onRecordStart}>
          <label htmlFor="p1">Configure start position</label>
          <span>
            {position.start.loading ? (
              <LoadingSpinner />
            ) : (
              <>
                {!!position.start.pos[0] ? (
                  <Position axis={position.start.pos} />
                ) : (
                  <RecordLabel />
                )}
              </>
            )}
          </span>
        </button>
        <button className="row-button" onClick={onRecordEnd}>
          <label htmlFor="p2">Configure end position</label>
          <span>
            {position.end.loading ? (
              <LoadingSpinner />
            ) : (
              <>
                {!!position.end.pos[0] ? (
                  <Position axis={position.end.pos} />
                ) : (
                  <RecordLabel />
                )}
              </>
            )}
          </span>
        </button>
      </div>
      <div className="separator"></div>
      <button
        disabled={!validPosition}
        className="primary"
        type="button"
        onClick={onInitAfk}
      >
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: 4,
          }}
        >
          Run
        </div>
      </button>
    </div>
  );
};

export default App;
