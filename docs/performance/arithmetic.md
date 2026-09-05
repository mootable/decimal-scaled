# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
<style>
:root{--fam-1:#2a78d6;--fam-2:#eb6834;--fam-3:#1baf7a}
[data-md-color-scheme="slate"]{--fam-1:#3987e5;--fam-2:#d95926;--fam-3:#199e70}
@media (prefers-color-scheme:dark){:root:not([data-md-color-scheme="default"]){
  --fam-1:#3987e5;--fam-2:#d95926;--fam-3:#199e70}}
.perf-chart{position:relative}
.perf-chart>input.fam-toggle{position:absolute;width:1px;height:1px;opacity:0;
  pointer-events:none;margin:0}
.perf-chart>input.fam-toggle:nth-of-type(1):not(:checked)~figure .fam-1,
.perf-chart>input.fam-toggle:nth-of-type(2):not(:checked)~figure .fam-2,
.perf-chart>input.fam-toggle:nth-of-type(3):not(:checked)~figure .fam-3,
.perf-chart>input.fam-toggle:nth-of-type(4):not(:checked)~figure .fam-avg{
  display:none}
.perf-chart .fam-legend{display:flex;flex-wrap:wrap;gap:.35rem .9rem;
  justify-content:center;margin-top:.25rem}
.perf-chart .fam-key{cursor:pointer;user-select:none;font-size:.7rem;
  display:inline-flex;align-items:center;gap:.3rem;opacity:.45;
  border-bottom:1px dotted currentColor}
.perf-chart .fam-key::before{content:"";width:.75rem;height:0;
  border-top:2px solid currentColor}
.perf-chart .fam-key.k1{color:var(--fam-1)}
.perf-chart .fam-key.k2{color:var(--fam-2)}
.perf-chart .fam-key.k3{color:var(--fam-3)}
.perf-chart>input.fam-toggle:nth-of-type(1):checked~figure .fam-key.k1,
.perf-chart>input.fam-toggle:nth-of-type(2):checked~figure .fam-key.k2,
.perf-chart>input.fam-toggle:nth-of-type(3):checked~figure .fam-key.k3,
.perf-chart>input.fam-toggle:nth-of-type(4):checked~figure .fam-key.kavg{
  opacity:1;border-bottom-style:solid}
.perf-chart .fam-key.kavg::before{border-top-style:dashed}
</style>

### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.935 ns | 0.937 ns | 1.04 ns | 0.936 ns | 0.935 ns |
| D38 | 1.82 ns | 1.62 ns | 1.03 ns | 1.25 ns | 1.63 ns |
| D57 | 2.26 ns | 2.5 ns | 2.25 ns | 2.25 ns | 2.5 ns |
| D76 | 1.48 ns | 3.48 ns | 3.48 ns | 2.7 ns | 3.09 ns |
| D115 | 4.4 ns | 4.4 ns | 2.86 ns | 3.34 ns | 4.99 ns |
| D153 | 3.5 ns | 6.62 ns | 6.65 ns | 5.91 ns | 6.63 ns |
| D230 | 11.9 ns | 10.2 ns | 15.3 ns | 14 ns | 13.9 ns |
| D307 | 15.2 ns | 15.9 ns | 19.6 ns | 18.5 ns | 16.2 ns |
| D462 | 29.8 ns | 32.8 ns | 28.7 ns | 32.6 ns | 42.7 ns |
| D616 | 70.7 ns | 49.8 ns | 45.1 ns | 45.1 ns | 57.8 ns |
| D924 | 61.9 ns | 75 ns | 76.6 ns | 84.8 ns | 84.9 ns |
| D1232 | 106 ns | 83.7 ns | 96.5 ns | 107 ns | 83.5 ns |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,161.5 88.2,147.0 124.4,142.3 160.5,151.5 196.7,127.8 232.9,132.8 269.1,106.2 305.3,100.9 341.5,86.3 377.6,67.5 413.8,70.4 450.0,58.7 450.0,63.9 413.8,63.6 377.6,71.9 341.5,78.5 305.3,99.6 269.1,102.8 232.9,118.9 196.7,125.1 160.5,135.5 124.4,140.1 88.2,149.3 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.5 88.2,147.0 124.4,142.3 160.5,151.5 196.7,127.8 232.9,132.8 269.1,106.2 305.3,100.9 341.5,86.3 377.6,67.5 413.8,70.4 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,149.5 124.4,140.1 160.5,132.9 196.7,127.8 232.9,119.0 269.1,109.6 305.3,99.9 341.5,84.2 377.6,75.1 413.8,66.3 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.2 88.2,159.4 124.4,142.4 160.5,132.9 196.7,137.2 232.9,118.8 269.1,100.7 305.3,95.4 341.5,87.1 377.6,77.3 413.8,65.8 450.0,60.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,155.2 124.4,142.4 160.5,138.4 196.7,133.8 232.9,121.4 269.1,102.7 305.3,96.6 341.5,84.3 377.6,77.3 413.8,63.6 450.0,58.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.3 124.4,140.1 160.5,135.5 196.7,125.1 232.9,118.9 269.1,102.8 305.3,99.6 341.5,78.5 377.6,71.9 413.8,63.6 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.95 ns | 5.93 ns | 6.35 ns | 7.84 ns | 8.12 ns |
| D38 | 12.7 ns | 11.3 ns | 12.1 ns | 54.7 ns | 59.4 ns |
| D57 | 23.5 ns | 34.6 ns | 68.1 ns | 111 ns | 120 ns |
| D76 | 15.1 ns | 65.4 ns | 83.6 ns | 100 ns | 141 ns |
| D115 | 42.7 ns | 83.4 ns | 71.3 ns | 181 ns | 253 ns |
| D153 | 32.4 ns | 115 ns | 156 ns | 241 ns | 353 ns |
| D230 | 72.4 ns | 86.8 ns | 253 ns | 392 ns | 570 ns |
| D307 | 103 ns | 173 ns | 400 ns | 591 ns | 872 ns |
| D462 | 190 ns | 473 ns | 682 ns | 1.15 µs | 1.19 µs |
| D616 | 251 ns | 546 ns | 1.04 µs | 1.87 µs | 2.17 µs |
| D924 | 263 ns | 1.09 µs | 2.06 µs | 2.85 µs | 4.68 µs |
| D1232 | 540 ns | 1.54 µs | 3.55 µs | 4.65 µs | 6.06 µs |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,171.3 88.2,154.8 124.4,141.5 160.5,151.1 196.7,128.5 232.9,134.5 269.1,117.0 305.3,109.4 341.5,96.1 377.6,90.1 413.8,89.0 450.0,73.4 450.0,20.9 413.8,26.5 377.6,43.2 341.5,56.2 305.3,63.0 269.1,72.2 232.9,82.6 196.7,89.8 160.5,102.6 124.4,106.1 88.2,121.3 52.0,164.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.3 88.2,154.8 124.4,141.5 160.5,151.1 196.7,128.5 232.9,134.5 269.1,117.0 305.3,109.4 341.5,96.1 377.6,90.1 413.8,89.0 450.0,73.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.3 88.2,157.3 124.4,133.1 160.5,119.2 196.7,113.9 232.9,107.0 269.1,113.1 305.3,98.1 341.5,76.2 377.6,73.2 413.8,58.2 450.0,50.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.9 88.2,155.9 124.4,118.3 160.5,113.9 196.7,117.4 232.9,100.4 269.1,89.9 305.3,79.9 341.5,68.3 377.6,59.1 413.8,44.3 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.3 88.2,123.1 124.4,107.6 160.5,109.9 196.7,97.1 232.9,90.9 269.1,80.3 305.3,71.4 341.5,56.9 377.6,46.4 413.8,37.2 450.0,26.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.5 88.2,121.3 124.4,106.1 160.5,102.6 196.7,89.8 232.9,82.6 269.1,72.2 305.3,63.0 341.5,56.2 377.6,43.2 413.8,26.5 450.0,20.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.885 ns | 2.87 ns | 3.02 ns | 4.97 ns | 5.26 ns |
| D38 | 3.95 ns | 14.9 ns | 20.7 ns | 21.6 ns | 26.5 ns |
| D57 | 6.24 ns | 21.6 ns | 33 ns | 72.5 ns | 76.9 ns |
| D76 | 2.89 ns | 38.2 ns | 46.9 ns | 64.8 ns | 99.5 ns |
| D115 | 13.3 ns | 49.8 ns | 58.1 ns | 180 ns | 253 ns |
| D153 | 10.4 ns | 55 ns | 119 ns | 236 ns | 395 ns |
| D230 | 22 ns | 69.7 ns | 368 ns | 503 ns | 989 ns |
| D307 | 42.1 ns | 128 ns | 508 ns | 1.03 µs | 1.16 µs |
| D462 | 103 ns | 459 ns | 1.25 µs | 1.87 µs | 2.09 µs |
| D616 | 114 ns | 591 ns | 1.74 µs | 2.72 µs | 3.89 µs |
| D924 | 106 ns | 1.49 µs | 3.02 µs | 5.42 µs | 8.28 µs |
| D1232 | 194 ns | 1.88 µs | 4.66 µs | 9.02 µs | 11.1 µs |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,178.4 88.2,156.8 124.4,150.2 160.5,161.3 196.7,139.2 232.9,142.7 269.1,131.9 305.3,122.5 341.5,109.6 377.6,108.0 413.8,109.2 450.0,100.4 450.0,41.9 413.8,46.1 377.6,57.0 341.5,66.0 305.3,74.5 269.1,76.8 232.9,90.1 196.7,96.6 160.5,110.1 124.4,113.8 88.2,129.2 52.0,152.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,156.8 124.4,150.2 160.5,161.3 196.7,139.2 232.9,142.7 269.1,131.9 305.3,122.5 341.5,109.6 377.6,108.0 413.8,109.2 450.0,100.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,137.6 124.4,132.2 160.5,123.9 196.7,120.1 232.9,118.7 269.1,115.2 305.3,106.4 341.5,87.9 377.6,84.3 413.8,70.9 450.0,67.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.7 88.2,132.8 124.4,126.1 160.5,121.0 196.7,117.9 232.9,107.5 269.1,91.1 305.3,86.5 341.5,73.5 377.6,68.7 413.8,60.7 450.0,54.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,132.2 124.4,114.6 160.5,116.3 196.7,101.5 232.9,97.6 269.1,86.6 305.3,76.3 341.5,67.6 377.6,62.2 413.8,52.2 450.0,44.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.6 88.2,129.2 124.4,113.8 160.5,110.1 196.7,96.6 232.9,90.1 269.1,76.8 305.3,74.5 341.5,66.0 377.6,57.0 413.8,46.1 450.0,41.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.622 ns | 0.622 ns | 0.346 ns | 0.622 ns | 0.622 ns |
| D38 | 1.45 ns | 1.33 ns | 1.1 ns | 1.12 ns | 1.32 ns |
| D57 | 1.87 ns | 1.74 ns | 1.87 ns | 1.87 ns | 1.74 ns |
| D76 | 1.09 ns | 2.17 ns | 2.16 ns | 1.68 ns | 2.49 ns |
| D115 | 2.83 ns | 2.83 ns | 2.12 ns | 2.78 ns | 3.55 ns |
| D153 | 2.43 ns | 4.22 ns | 4.6 ns | 4.3 ns | 4.6 ns |
| D230 | 5.16 ns | 3.83 ns | 7.43 ns | 7.18 ns | 7.18 ns |
| D307 | 9.52 ns | 7.7 ns | 12.4 ns | 11 ns | 7.91 ns |
| D462 | 15.4 ns | 17 ns | 15.2 ns | 16.6 ns | 24.3 ns |
| D616 | 23.6 ns | 17.2 ns | 20.3 ns | 20.2 ns | 20.3 ns |
| D924 | 43.9 ns | 76.3 ns | 76.6 ns | 84.6 ns | 84.8 ns |
| D1232 | 54.3 ns | 52.2 ns | 61.6 ns | 71.3 ns | 51.7 ns |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,157.1 88.2,132.7 124.4,125.2 160.5,140.7 196.7,113.2 232.9,117.6 269.1,95.8 305.3,78.1 341.5,64.2 377.6,51.8 413.8,33.8 450.0,27.7 450.0,29.1 413.8,14.8 377.6,56.2 341.5,51.0 305.3,83.5 269.1,86.3 232.9,99.2 196.7,106.6 160.5,117.0 124.4,127.3 88.2,135.2 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.1 88.2,132.7 124.4,125.2 160.5,140.7 196.7,113.2 232.9,117.6 269.1,95.8 305.3,78.1 341.5,64.2 377.6,51.8 413.8,33.8 450.0,27.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,135.1 124.4,127.3 160.5,120.9 196.7,113.3 232.9,101.6 269.1,104.5 305.3,84.2 341.5,61.3 377.6,61.0 413.8,17.8 450.0,28.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,174.1 88.2,140.6 124.4,125.2 160.5,121.0 196.7,121.5 232.9,99.2 269.1,85.3 305.3,70.5 341.5,64.6 377.6,56.2 413.8,17.7 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,140.0 124.4,125.2 160.5,128.3 196.7,113.7 232.9,101.1 269.1,86.3 305.3,74.0 341.5,61.9 377.6,56.3 413.8,14.8 450.0,19.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.2 124.4,127.3 160.5,117.0 196.7,106.6 232.9,99.2 269.1,86.3 305.3,83.5 341.5,51.0 377.6,56.2 413.8,14.8 450.0,29.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 1.87 ns | 1.73 ns | 2.18 ns | 2.18 ns |
| D38 | 3.54 ns | 3.48 ns | 2.53 ns | 2.3 ns | 3.17 ns |
| D57 | 7.17 ns | 8.09 ns | 7.17 ns | 7.16 ns | 8.09 ns |
| D76 | 3.97 ns | 9.83 ns | 9.52 ns | 7.37 ns | 8.59 ns |
| D115 | 12.7 ns | 12.8 ns | 7.59 ns | 9.28 ns | 14.4 ns |
| D153 | 9.36 ns | 20 ns | 20 ns | 16.3 ns | 20.1 ns |
| D230 | 28.4 ns | 16.8 ns | 36.2 ns | 31.9 ns | 31.8 ns |
| D307 | 37.2 ns | 31.4 ns | 47.9 ns | 42.6 ns | 32.5 ns |
| D462 | 76.6 ns | 86.9 ns | 77.8 ns | 85.2 ns | 77.6 ns |
| D616 | 97.5 ns | 78.1 ns | 79.4 ns | 78 ns | 84.1 ns |
| D924 | 73.2 ns | 102 ns | 103 ns | 98.1 ns | 92.6 ns |
| D1232 | 155 ns | 112 ns | 127 ns | 129 ns | 99.3 ns |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,191.9 88.2,173.4 124.4,153.0 160.5,170.1 196.7,136.3 232.9,145.3 269.1,113.1 305.3,105.3 341.5,84.4 377.6,77.4 413.8,85.7 450.0,64.0 450.0,76.9 413.8,78.9 377.6,81.7 341.5,84.0 305.3,109.2 269.1,109.8 232.9,123.2 196.7,132.8 160.5,147.7 124.4,149.5 88.2,176.6 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,173.4 124.4,153.0 160.5,170.1 196.7,136.3 232.9,145.3 269.1,113.1 305.3,105.3 341.5,84.4 377.6,77.4 413.8,85.7 450.0,64.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,173.9 124.4,149.5 160.5,143.8 196.7,136.3 232.9,123.2 269.1,128.3 305.3,110.2 341.5,80.7 377.6,83.8 413.8,76.0 450.0,73.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,194.2 88.2,183.2 124.4,153.0 160.5,144.8 196.7,151.3 232.9,123.2 269.1,106.1 305.3,97.9 341.5,83.9 377.6,83.3 413.8,75.9 450.0,69.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,185.9 124.4,153.0 160.5,152.2 196.7,145.5 232.9,129.3 269.1,109.7 305.3,101.4 341.5,81.3 377.6,83.9 413.8,77.2 450.0,69.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,176.6 124.4,149.5 160.5,147.7 196.7,132.8 232.9,123.2 269.1,109.8 305.3,109.2 341.5,84.0 377.6,81.7 413.8,78.9 450.0,76.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.25 ns | 1.25 ns | 1.05 ns | 1.25 ns | 1.25 ns |
| D38 | 1.81 ns | 1.61 ns | 0.96 ns | 1.12 ns | 1.61 ns |
| D57 | 2.27 ns | 2.5 ns | 2.27 ns | 2.27 ns | 2.5 ns |
| D76 | 1.58 ns | 3.45 ns | 3.45 ns | 2.67 ns | 3.1 ns |
| D115 | 4.9 ns | 4.89 ns | 3.6 ns | 4.03 ns | 5.57 ns |
| D153 | 4.66 ns | 8.47 ns | 8.47 ns | 7.63 ns | 8.49 ns |
| D230 | 13.7 ns | 11 ns | 17.6 ns | 16.1 ns | 16.1 ns |
| D307 | 19.5 ns | 18.2 ns | 25.2 ns | 23.4 ns | 18.8 ns |
| D462 | 37.6 ns | 40.6 ns | 37.1 ns | 41.3 ns | 49.8 ns |
| D616 | 71.6 ns | 49.9 ns | 46.2 ns | 45.1 ns | 58 ns |
| D924 | 68.7 ns | 75.1 ns | 75.3 ns | 84.8 ns | 84.7 ns |
| D1232 | 106 ns | 83.8 ns | 97.1 ns | 108 ns | 83.3 ns |

<figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,155.2 88.2,147.1 124.4,142.2 160.5,150.0 196.7,125.5 232.9,126.6 269.1,103.2 305.3,95.5 341.5,81.3 377.6,67.3 413.8,68.2 450.0,58.7 450.0,64.0 413.8,63.6 377.6,71.8 341.5,75.2 305.3,96.3 269.1,99.7 232.9,113.6 196.7,122.7 160.5,135.4 124.4,140.1 88.2,149.6 52.0,155.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,155.2 88.2,147.1 124.4,142.2 160.5,150.0 196.7,125.5 232.9,126.6 269.1,103.2 305.3,95.5 341.5,81.3 377.6,67.3 413.8,68.2 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,155.2 88.2,149.6 124.4,140.1 160.5,133.1 196.7,125.5 232.9,113.6 269.1,107.8 305.3,97.0 341.5,79.6 377.6,75.1 413.8,66.2 450.0,63.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.0 88.2,160.9 124.4,142.2 160.5,133.1 196.7,132.2 232.9,113.6 269.1,97.7 305.3,90.0 341.5,81.5 377.6,76.8 413.8,66.2 450.0,60.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.2 88.2,157.5 124.4,142.2 160.5,138.7 196.7,129.7 232.9,115.9 269.1,99.6 305.3,91.5 341.5,79.2 377.6,77.3 413.8,63.6 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.2 88.2,149.6 124.4,140.1 160.5,135.4 196.7,122.7 232.9,113.6 269.1,99.7 305.3,96.3 341.5,75.2 377.6,71.8 413.8,63.6 450.0,64.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption></figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
