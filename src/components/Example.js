import React from 'react';
import CodeBlock from '@theme/CodeBlock';
import Screenshot from './Screenshot';
import styles from './styles.module.css';

// Every example program, loaded as plain text straight from the Rust crate.
// The docs never contain a copy of the code: what you read is the file that is tested.
const sources = require.context('!!raw-loader!@site/examples/src/bin', false, /\.rs$/);

function load(name) {
  const key = `./${name}.rs`;
  if (!sources.keys().includes(key)) {
    throw new Error(`<Example name="${name}" />: examples/src/bin/${name}.rs does not exist`);
  }
  return sources(key).default.trimEnd();
}

/**
 * Cut one statement or item (a function, struct, impl block, `let`, call chain...)
 * out of a source file.
 *
 * `item` is the text its first line starts with, for example "fn press".
 * `to` optionally extends the excerpt: it then runs up to and including the
 * statement whose first line starts with that text.
 */
function extract(source, item, name, to) {
  const lines = source.split('\n');
  const start = lines.findIndex((line) => line.trimStart().startsWith(item));
  if (start === -1) {
    throw new Error(`<Example name="${name}" item="${item}" />: no line starts with that text`);
  }

  let end = start;
  if (to) {
    end = lines.findIndex((line, i) => i > start && line.trimStart().startsWith(to));
    if (end === -1) {
      throw new Error(`<Example name="${name}" to="${to}" />: no later line starts with that text`);
    }
  }

  // Include doc comments and attributes directly above the item.
  let first = start;
  while (first > 0 && /^\s*(\/\/\/|#\[)/.test(lines[first - 1])) {
    first -= 1;
  }

  // Walk forward to the end of the statement: brackets balanced again, and the
  // line ends the way a statement or block does.
  let depth = 0;
  let last = start;
  for (let i = start; i < lines.length; i++) {
    const code = lines[i].replace(/"(\\.|[^"\\])*"/g, '""').replace(/\/\/.*$/, '').trimEnd();
    for (const ch of code) {
      if ('{[('.includes(ch)) depth += 1;
      else if ('}])'.includes(ch)) depth -= 1;
    }
    if (depth < 0) break; // we ran out of the enclosing block: stop before this line
    last = i;
    if (i >= end && depth === 0 && /[;}]$/.test(code)) break;
  }

  const picked = lines.slice(first, last + 1);
  const indent = Math.min(...picked.filter((l) => l.trim()).map((l) => l.match(/^ */)[0].length));
  return picked.map((l) => l.slice(indent)).join('\n');
}

/**
 * Show a tested example program.
 *
 *   <Example name="widget_button" />                 the whole file plus its screenshot
 *   <Example name="calc_2_logic" item="fn press" />  just one function, no screenshot
 *   <Example name="x" item="let a" to="ui.label" />  from one statement through another
 *   <Example name="widget_button" shot="other" />    use a different screenshot
 *   <Example name="hello_simple" shot={false} />     no screenshot
 */
export default function Example({name, item, to, shot, title, highlight}) {
  const full = load(name);
  const code = item ? extract(full, item, name, to) : full;
  const screenshot = shot === false ? null : shot ?? (item ? null : name);
  const blockTitle = title ?? (item ? `from src/bin/${name}.rs` : 'src/main.rs');

  return (
    <div className={styles.example}>
      <CodeBlock language="rust" title={blockTitle} metastring={highlight ? `{${highlight}}` : undefined}>
        {code}
      </CodeBlock>
      {screenshot && <Screenshot name={screenshot} />}
      {!item && (
        <p className={styles.run}>
          Tested example. Run it from the <code>examples</code> folder with{' '}
          <code>cargo run --bin {name}</code>
        </p>
      )}
    </div>
  );
}
