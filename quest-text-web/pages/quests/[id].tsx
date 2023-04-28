import React from "react";

interface Quest {
  Key: number;
  InternalId: string;
  PreviousQuests: number[];
  NextQuests: number[];
  NameEn: string;
  NameJa: string;
  TodosEn: string[];
  TodosJa: string[];
  JournalEn: string[];
  JournalJa: string[];
  DialogueEn: { Speaker: string; Text: string }[];
  DialogueJa: { Speaker: string; Text: string }[];
}

// pre-wrap: preserve newlines and other whitespace from the original HTML source
// (normally whitespace is collapsed in HTML)
// wordBreak: allow breaks even in the middle of some words
const wrappingBehavior = {
  whiteSpace: "pre-wrap",
  wordBreak: "break-word",
} as const;

function Quest({ quest }: { quest: Quest }) {
  return (
    <div>
      <h2>
        {quest.NameJa} / {quest.NameEn}
      </h2>
      {/* TODO: add quest names to links */}
      <p>
        Next quests:{" "}
        {quest.NextQuests?.map((q: any) => (
          <a href={`/quests/${q}`}>{q}</a>
        ))}
      </p>
      <p>
        Previous quests:{" "}
        {quest.PreviousQuests.map((q: any) => (
          <a href={`/quests/${q}`}>{q}</a>
        ))}
      </p>

      <TwoTabs tab1Name="日本語" tab2Name="English">
        <TwoColumnsBiasFirst>
          <DialogLines lines={quest.DialogueJa} />
          <div>
            TODO
            <OtherLines lines={quest.TodosJa} />
            JOURNAL
            <OtherLines lines={quest.JournalJa} />
          </div>
        </TwoColumnsBiasFirst>
        <TwoColumnsBiasFirst>
          <DialogLines lines={quest.DialogueEn} />
          <div>
            TODO
            <OtherLines lines={quest.TodosEn} />
            JOURNAL
            <OtherLines lines={quest.JournalEn} />
          </div>
        </TwoColumnsBiasFirst>
      </TwoTabs>
    </div>
  );
}

function TwoTabs(props: {
  children: React.ReactNode[];
  tab1Name: string;
  tab2Name: string;
}) {
  return (
    <>
      <style jsx>
        {`
          /* based on https://codepen.io/MPDoctor/pen/mpJdYe

    the underlying mechanism is a set of hidden radio inputs.

    Since the radio inputs all have the same name attribute, only one of them
    can be checked at a time.

    The clever trick is to find a CSS rule which matches the Nth tab when the Nth radio button is checked.
    */
          .tabbed {
            overflow-x: hidden; /* so we could easily hide the radio inputs */
            margin: 32px 0;
            padding-bottom: 16px;
            border-bottom: 1px solid #ccc;
          }

          /* We don't show the radio buttons themselves - only the tabs (labels) which control them, and the tab contents that gets displayed */
          .tabbed [type="radio"] {
            display: none;
          }

          .tabs {
            display: flex;
            align-items: stretch;
            list-style: none;
            padding: 0;
            border-bottom: 1px solid #ccc;
            gap: 10px;
          }
          .tab > label {
            display: block;
            margin-bottom: -1px;
            height: 16px;
            padding: 12px 15px;

            border: 1px solid #ccc;

            font-size: 12px;
            font-weight: 600;

            text-transform: uppercase;
            cursor: pointer;
          }

          /* As we cannot replace the numbers with variables or calls to element properties, if we need more tabs then we have to add more css selectors here */
          .tabbed
            [type="radio"]:nth-of-type(1):checked
            ~ .tabs
            .tab:nth-of-type(1)
            label,
          .tabbed
            [type="radio"]:nth-of-type(2):checked
            ~ .tabs
            .tab:nth-of-type(2)
            label {
            border-bottom-color: #555;
          }

          /* Tabs are hidden by default */
          .tab-content {
            display: none;
          }

          /* Tabs are shown when their corresponding radio input is checked */
          .tabbed
            [type="radio"]:nth-of-type(1):checked
            ~ .tab-content:nth-of-type(1),
          .tabbed
            [type="radio"]:nth-of-type(2):checked
            ~ .tab-content:nth-of-type(2) {
            display: block;
          }
        `}
      </style>
      <div className="tabbed">
        <input type="radio" id="d-ja" name="css-tabs" defaultChecked />
        <input type="radio" id="d-en" name="css-tabs" />

        <ul className="tabs">
          <li className="tab">
            <label htmlFor="d-ja">{props.tab1Name}</label>
          </li>
          <li className="tab">
            <label htmlFor="d-en">{props.tab2Name}</label>
          </li>
        </ul>
        {props.children.map((c) => (
          <div className="tab-content">{c}</div>
        ))}
      </div>
    </>
  );
}

const dataFolder = "c:\\temp\\all-quest-texts\\";

function DialogLines(props: { lines: { Speaker: string; Text: string }[] }) {
  return (
    <div
      style={{
        display: "grid",
        gridAutoFlow: "column",
        gridTemplateColumns: "auto auto",
        gridTemplateRows: `repeat(${props.lines.length}, auto)`,
        gap: "10px",
      }}
    >
      {props.lines.map((qt: any) => (
        <div style={{ textAlign: "right" }}>{qt.Speaker}</div>
      ))}
      {props.lines.map((qt: any) => (
        <div style={{ ...wrappingBehavior }}>{qt.Text}</div>
      ))}
    </div>
  );
}

function OtherLines(props: { lines: string[] }) {
  return (
    <ul>
      {props.lines.map((l) => (
        <li style={{ ...wrappingBehavior }}>{l}</li>
      ))}
    </ul>
  );
}

function TwoColumnsBiasFirst(props: { children: React.ReactNode[] }) {
  return (
    <div
      style={{
        display: "grid",
        gridAutoFlow: "column",
        gridTemplateColumns: "2fr 1fr",
        gap: "10px",
      }}
    >
      {props.children}
    </div>
  );
}

function getStaticPaths() {
  const fs = require("fs");
  const path = require("path");
  const filenamesInDataFolder = fs.readdirSync(dataFolder);
  const filenamesWithoutExtensions = filenamesInDataFolder.map(
    (f: string) => path.parse(f).name
  );
  return {
    paths: filenamesWithoutExtensions.map((f: string) => ({
      params: { id: f },
    })),
    fallback: false,
  };
}

function getStaticProps({ params }: { params: any }) {
  const fs = require("fs");
  const data = JSON.parse(fs.readFileSync(`${dataFolder}${params.id}.json`));
  return {
    props: {
      quest: data,
    },
  };
}

export { getStaticPaths, getStaticProps };

export default Quest;
