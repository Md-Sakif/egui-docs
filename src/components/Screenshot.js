import React from 'react';
import {ThemedComponent} from '@docusaurus/theme-common';
import useBaseUrl from '@docusaurus/useBaseUrl';
import sizes from '@site/src/generated/screenshot-sizes.json';
import styles from './styles.module.css';

/**
 * A screenshot of an example.
 *
 * Most are rendered by the example tests (examples/tests/screenshots.rs), once
 * per color mode and at twice the size for sharpness. They are shown at half
 * their pixel size.
 *
 *   <Screenshot name="widget_button" />
 *
 * A few are captured from a real window or browser instead. Those exist once,
 * at normal size:
 *
 *   <Screenshot name="web_example" single />
 */
export default function Screenshot({name, alt, single}) {
  const base = useBaseUrl('/img/examples/');
  const description = alt ?? `Screenshot of the ${name} example`;

  const image = (file, scale, className) => {
    const size = sizes[file];
    if (!size) {
      throw new Error(`<Screenshot name="${name}" />: static/img/examples/${file} does not exist`);
    }
    return (
      <img
        className={className}
        src={base + file}
        width={size[0] / scale}
        height={size[1] / scale}
        alt={description}
        loading="lazy"
      />
    );
  };

  return (
    <figure className={styles.screenshot}>
      {single ? (
        image(`${name}.png`, 1)
      ) : (
        <ThemedComponent>
          {({theme, className}) => image(`${name}.${theme}.png`, 2, className)}
        </ThemedComponent>
      )}
    </figure>
  );
}
