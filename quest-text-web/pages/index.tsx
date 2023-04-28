import React from "react";

interface RedoGroup {
  NameEn: string;
  NameJa: string;
  Chapters: RedoChapter[];
}

interface RedoChapter {
  NameEn: string;
  NameJa: string;
  Quests: RedoQuest[];
}

interface RedoQuest {
  NameEn: string;
  NameJa: string;
  Key: number;
}

function HomePage(props: { data: RedoGroup[] }) {
  return (
    <div style={{ columns: "14rem auto", columnFill: 'auto', height: '90vh' }}>
      {props.data.map((g) => (
        <div style={{breakInside:'avoid'}}>
          <h2>{g.NameEn}</h2>
          {g.Chapters.map((c) => (
            <details>
              <summary>{c.NameEn}</summary>
              <ul>
                {c.Quests.map((q) => (
                  <li>
                    <a href={`/quests/${q.Key}`}>{q.NameEn}</a>
                  </li>
                ))}
              </ul>
            </details>
          ))}
        </div>
      ))}
    </div>
  );
}

const dataFolder = "c:\\temp\\";

function getStaticProps() {
  const fs = require("fs");
  const data = JSON.parse(fs.readFileSync(`${dataFolder}redo-quest-keys.json`));
  return {
    props: {
      data,
    },
  };
}

export { getStaticProps };

export default HomePage;
