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

function getStaticPaths() {
  const data = require("../../public/all-quest-texts.data.json");
  return {
    paths: Object.keys(data).map((id) => ({ params: { id } })),
    fallback: false,
  };
}

function getStaticProps({ params }: { params: any }) {
  const data = require("../../public/all-quest-texts.data.json");
  return {
    props: {
      quest: data[params.id],
    },
  };
}

export { getStaticPaths, getStaticProps };

export default Quest;
