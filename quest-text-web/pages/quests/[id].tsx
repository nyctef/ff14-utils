import React from "react";

function Quest({ quest }: { quest: any }) {
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
      <ul>
        {quest.TodosEn.map((qt: any) => (
          <li>{qt}</li>
        ))}
      </ul>
      <ul>
        {quest.JournalEn.map((qt: any) => (
          <li style={{ whiteSpace: "pre-wrap" }}>{qt}</li>
        ))}
      </ul>
      <div
        style={{
          display: "grid",
          gridAutoFlow: "column",
          gridTemplateColumns: "200px auto",
          gridTemplateRows: `repeat(${quest.DialogueEn.length}, auto)`,
          gap: "10px",
        }}
      >
        {quest.DialogueEn.map((qt: any) => (
          <div style={{ textAlign: "right" }}>{qt.Speaker}</div>
        ))}
        {quest.DialogueEn.map((qt: any) => (
          <div style={{ whiteSpace: "pre-wrap" }}>{qt.Text}</div>
        ))}
      </div>
    </div>
  );
}

const dataFolder = "c:\\temp\\all-quest-texts\\";

function getStaticPaths() {
  const fs = require('fs');
  const path = require('path');
  const filenamesInDataFolder = fs.readdirSync(dataFolder);
  const filenamesWithoutExtensions = filenamesInDataFolder.map((f:string) => path.parse(f).name);
  return {
    paths: filenamesWithoutExtensions.map((f:string) => ({ params: { id: f } })),
    fallback: false,
  };
}

function getStaticProps({ params }: { params: any }) {
  const fs = require('fs');
  const data = JSON.parse(fs.readFileSync(`${dataFolder}${params.id}.json`));
  return {
    props: {
      quest: data,
    },
  };
}

export { getStaticPaths, getStaticProps };

export default Quest;
