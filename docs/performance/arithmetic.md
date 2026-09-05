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

| Row | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| `add@base` D18 | 1.25 ns | 1.25 ns | 1.25 ns | 1.06 ns | 1.25 ns |
| `add@base` D38 | 1.81 ns | 1.81 ns | 1.62 ns | 1.42 ns | 0.827 ns |
| `add@base` D57 | 2.51 ns | 2.5 ns | 1.94 ns | 1.61 ns | 2.25 ns |
| `add@base` D76 | 3.09 ns | 1.83 ns | 1.56 ns | 1.8 ns | 1.81 ns |
| `add@base` D115 | 5 ns | 5 ns | 2.86 ns | 3.91 ns | 4.99 ns |
| `add@base` D153 | 5.9 ns | 4.5 ns | 4.02 ns | 3.52 ns | 4.47 ns |
| `add@base` D230 | 11.9 ns | 13.9 ns | 13.9 ns | 13.9 ns | 15.4 ns |
| `add@base` D307 | 15.2 ns | 18.6 ns | 18.5 ns | 18.5 ns | 18.5 ns |
| `add@base` D462 | 28.9 ns | 40.8 ns | 29 ns | 25.9 ns | 30 ns |
| `add@base` D616 | 49.6 ns | 49.4 ns | 38.6 ns | 47.3 ns | 61 ns |
| `add@base` D924 | 51.3 ns | 85.1 ns | 78.9 ns | 74.6 ns | 71.4 ns |
| `add@base` D1232 | 95.1 ns | 64.8 ns | 107 ns | 95.1 ns | 107 ns |

<div class="perf-chart"><input type="checkbox" class="fam-toggle" id="f-add-0" checked><figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,155.2 88.2,147.1 124.4,140.0 160.5,135.5 196.7,125.0 232.9,121.5 269.1,106.2 305.3,100.9 341.5,87.0 377.6,75.2 413.8,74.5 450.0,61.1 450.0,58.5 413.8,67.3 377.6,70.7 341.5,86.1 305.3,96.6 269.1,100.7 232.9,127.5 196.7,125.1 160.5,147.1 124.4,142.4 88.2,164.1 52.0,155.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,155.2 88.2,147.1 124.4,140.0 160.5,135.5 196.7,125.0 232.9,121.5 269.1,106.2 305.3,100.9 341.5,87.0 377.6,75.2 413.8,74.5 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,155.2 88.2,147.1 124.4,140.1 160.5,146.8 196.7,125.0 232.9,127.3 269.1,102.8 305.3,96.6 341.5,79.5 377.6,75.3 413.8,63.5 450.0,69.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.2 88.2,149.6 124.4,145.6 160.5,150.4 196.7,137.2 232.9,129.8 269.1,102.9 305.3,96.6 341.5,86.9 377.6,80.7 413.8,65.2 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,152.4 124.4,149.6 160.5,147.2 196.7,130.4 232.9,132.7 269.1,102.8 305.3,96.6 341.5,89.3 377.6,76.3 413.8,66.4 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.2 88.2,164.1 124.4,142.4 160.5,147.1 196.7,125.1 232.9,127.5 269.1,100.7 305.3,96.6 341.5,86.1 377.6,70.7 413.8,67.3 450.0,58.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). solid = scale 0 and max, dashed = the intermediate band-edge scales.</figcaption><div class="fam-legend"><label class="fam-key k1" for="f-add-0">add@base</label></div></figure></div>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Row | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| `div@base` D18 | 5.97 ns | 5.93 ns | 5.52 ns | 8.79 ns | 7.89 ns |
| `div@base` D38 | 12.7 ns | 12.6 ns | 15.7 ns | 53.7 ns | 39.5 ns |
| `div@base` D57 | 22.8 ns | 34.6 ns | 58.4 ns | 93.9 ns | 112 ns |
| `div@base` D76 | 26.3 ns | 38.7 ns | 43.8 ns | 85.4 ns | 107 ns |
| `div@base` D115 | 46.8 ns | 88.1 ns | 71.1 ns | 157 ns | 253 ns |
| `div@base` D153 | 54.9 ns | 103 ns | 88 ns | 154 ns | 285 ns |
| `div@base` D230 | 73.3 ns | 154 ns | 235 ns | 392 ns | 628 ns |
| `div@base` D307 | 103 ns | 234 ns | 360 ns | 590 ns | 886 ns |
| `div@base` D462 | 192 ns | 485 ns | 685 ns | 763 ns | 1.14 µs |
| `div@base` D616 | 242 ns | 616 ns | 720 ns | 1.82 µs | 2.42 µs |
| `div@base` D924 | 221 ns | 1.23 µs | 1.81 µs | 2.54 µs | 3.67 µs |
| `div@base` D1232 | 527 ns | 1.1 µs | 3.81 µs | 3.95 µs | 7.89 µs |

<div class="perf-chart"><input type="checkbox" class="fam-toggle" id="f-div-0" checked><figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,171.2 88.2,154.8 124.4,142.1 160.5,139.0 196.7,126.5 232.9,123.0 269.1,116.7 305.3,109.4 341.5,95.8 377.6,90.8 413.8,92.8 450.0,73.9 450.0,15.1 413.8,31.8 377.6,40.8 341.5,57.1 305.3,62.6 269.1,70.1 232.9,87.3 196.7,89.8 160.5,108.5 124.4,107.5 88.2,130.2 52.0,165.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.2 88.2,154.8 124.4,142.1 160.5,139.0 196.7,126.5 232.9,123.0 269.1,116.7 305.3,109.4 341.5,95.8 377.6,90.8 413.8,92.8 450.0,73.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.3 88.2,154.9 124.4,133.1 160.5,130.6 196.7,112.7 232.9,109.4 269.1,100.7 305.3,91.5 341.5,75.7 377.6,70.5 413.8,55.5 450.0,58.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.9 88.2,150.2 124.4,121.7 160.5,127.9 196.7,117.4 232.9,112.8 269.1,91.5 305.3,82.2 341.5,68.2 377.6,67.1 413.8,47.1 450.0,30.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.8 88.2,123.5 124.4,111.4 160.5,113.4 196.7,100.2 232.9,100.6 269.1,80.3 305.3,71.5 341.5,65.9 377.6,47.0 413.8,39.8 450.0,30.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.2 88.2,130.2 124.4,107.5 160.5,108.5 196.7,89.8 232.9,87.3 269.1,70.1 305.3,62.6 341.5,57.1 377.6,40.8 413.8,31.8 450.0,15.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). solid = scale 0 and max, dashed = the intermediate band-edge scales.</figcaption><div class="fam-legend"><label class="fam-key k1" for="f-div-0">div@base</label></div></figure></div>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Row | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| `mul@base` D18 | 0.89 ns | 2.87 ns | 3.13 ns | 5.01 ns | 5.06 ns |
| `mul@base` D38 | 3.97 ns | 13.7 ns | 24.7 ns | 31.8 ns | 14.7 ns |
| `mul@base` D57 | 4.22 ns | 21.7 ns | 26.7 ns | 60.2 ns | 71.3 ns |
| `mul@base` D76 | 8.41 ns | 25.3 ns | 26.3 ns | 56.2 ns | 73.1 ns |
| `mul@base` D115 | 13.5 ns | 54.6 ns | 58 ns | 166 ns | 255 ns |
| `mul@base` D153 | 18.1 ns | 46.5 ns | 67.3 ns | 156 ns | 317 ns |
| `mul@base` D230 | 21.6 ns | 116 ns | 331 ns | 504 ns | 1.02 µs |
| `mul@base` D307 | 42.3 ns | 170 ns | 465 ns | 1.05 µs | 1.38 µs |
| `mul@base` D462 | 91.8 ns | 463 ns | 1.27 µs | 1.1 µs | 1.81 µs |
| `mul@base` D616 | 88.5 ns | 664 ns | 1.12 µs | 2.71 µs | 4.19 µs |
| `mul@base` D924 | 94.8 ns | 1.59 µs | 2.54 µs | 5.04 µs | 6.52 µs |
| `mul@base` D1232 | 199 ns | 1.24 µs | 5.1 µs | 8.26 µs | 14.2 µs |

<div class="perf-chart"><input type="checkbox" class="fam-toggle" id="f-mul-0" checked><figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,178.4 88.2,156.7 124.4,155.8 160.5,145.8 196.7,139.0 232.9,134.7 269.1,132.2 305.3,122.5 341.5,111.2 377.6,111.8 413.8,110.8 450.0,100.1 450.0,38.2 413.8,49.5 377.6,55.9 341.5,68.1 305.3,72.0 269.1,76.3 232.9,93.3 196.7,96.5 160.5,114.5 124.4,114.9 88.2,137.7 52.0,153.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,156.7 124.4,155.8 160.5,145.8 196.7,139.0 232.9,134.7 269.1,132.2 305.3,122.5 341.5,111.2 377.6,111.8 413.8,110.8 450.0,100.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,138.8 124.4,132.1 160.5,129.9 196.7,118.8 232.9,121.1 269.1,107.8 305.3,102.3 341.5,87.8 377.6,82.6 413.8,69.9 450.0,73.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.2 88.2,130.2 124.4,129.1 160.5,129.3 196.7,117.9 232.9,115.7 269.1,92.7 305.3,87.7 341.5,73.2 377.6,75.0 413.8,63.1 450.0,53.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.3 88.2,126.6 124.4,117.4 160.5,118.3 196.7,102.7 232.9,103.6 269.1,86.6 305.3,76.0 341.5,75.2 377.6,62.2 413.8,53.3 450.0,46.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.2 88.2,137.7 124.4,114.9 160.5,114.5 196.7,96.5 232.9,93.3 269.1,76.3 305.3,72.0 341.5,68.1 377.6,55.9 413.8,49.5 450.0,38.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). solid = scale 0 and max, dashed = the intermediate band-edge scales.</figcaption><div class="fam-legend"><label class="fam-key k1" for="f-mul-0">mul@base</label></div></figure></div>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Row | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| `neg@base` D18 | 0.622 ns | 0.622 ns | 0.622 ns | 0.703 ns | 0.622 ns |
| `neg@base` D38 | 1.45 ns | 1.45 ns | 1.33 ns | 1.12 ns | 0.881 ns |
| `neg@base` D57 | 1.74 ns | 1.74 ns | 1.35 ns | 1.44 ns | 1.69 ns |
| `neg@base` D76 | 2.18 ns | 1.47 ns | 1.29 ns | 1.45 ns | 1.74 ns |
| `neg@base` D115 | 3.16 ns | 3.17 ns | 2.12 ns | 2.76 ns | 3.55 ns |
| `neg@base` D153 | 3.79 ns | 2.91 ns | 2.71 ns | 2.65 ns | 3.29 ns |
| `neg@base` D230 | 5.16 ns | 5.97 ns | 7.18 ns | 7.18 ns | 7.43 ns |
| `neg@base` D307 | 9.53 ns | 11.1 ns | 11 ns | 11 ns | 11 ns |
| `neg@base` D462 | 15 ns | 16.6 ns | 14.9 ns | 12.4 ns | 14.4 ns |
| `neg@base` D616 | 19 ns | 20.1 ns | 15.5 ns | 23.2 ns | 21.9 ns |
| `neg@base` D924 | 30.9 ns | 84.8 ns | 71.6 ns | 76.1 ns | 69.2 ns |
| `neg@base` D1232 | 47.2 ns | 38.8 ns | 69.8 ns | 61.8 ns | 69.8 ns |

<div class="perf-chart"><input type="checkbox" class="fam-toggle" id="f-neg-0" checked><figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,157.1 88.2,132.6 124.4,127.3 160.5,120.7 196.7,110.0 232.9,104.8 269.1,95.8 305.3,78.1 341.5,64.9 377.6,58.1 413.8,44.0 450.0,31.8 450.0,20.4 413.8,20.7 377.6,53.9 341.5,66.2 305.3,73.9 269.1,85.3 232.9,108.8 196.7,106.6 160.5,127.2 124.4,128.2 88.2,147.0 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.1 88.2,132.6 124.4,127.3 160.5,120.7 196.7,110.0 232.9,104.8 269.1,95.8 305.3,78.1 341.5,64.9 377.6,58.1 413.8,44.0 450.0,31.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,132.7 124.4,127.3 160.5,132.1 196.7,110.0 232.9,112.4 269.1,91.6 305.3,73.7 341.5,61.9 377.6,56.4 413.8,14.8 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.1 124.4,134.6 160.5,136.0 196.7,121.6 232.9,114.4 269.1,86.3 305.3,73.9 341.5,65.0 377.6,64.0 413.8,19.7 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,140.0 124.4,132.7 160.5,132.6 196.7,114.0 232.9,115.1 269.1,86.3 305.3,73.9 341.5,70.5 377.6,52.3 413.8,17.9 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,147.0 124.4,128.2 160.5,127.2 196.7,106.6 232.9,108.8 269.1,85.3 305.3,73.9 341.5,66.2 377.6,53.9 413.8,20.7 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). solid = scale 0 and max, dashed = the intermediate band-edge scales.</figcaption><div class="fam-legend"><label class="fam-key k1" for="f-neg-0">neg@base</label></div></figure></div>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Row | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| `rem@base` D18 | 1.87 ns | 1.87 ns | 1.87 ns | 2.46 ns | 2.18 ns |
| `rem@base` D38 | 3.92 ns | 4.05 ns | 3.49 ns | 2.73 ns | 1.68 ns |
| `rem@base` D57 | 8.09 ns | 8.1 ns | 6.27 ns | 4.27 ns | 7.18 ns |
| `rem@base` D76 | 8.44 ns | 5.34 ns | 4.4 ns | 4.85 ns | 4.83 ns |
| `rem@base` D115 | 14.4 ns | 14.4 ns | 7.56 ns | 11.2 ns | 14.4 ns |
| `rem@base` D153 | 16.4 ns | 12.3 ns | 9.36 ns | 9.36 ns | 12.1 ns |
| `rem@base` D230 | 28.4 ns | 31.5 ns | 34 ns | 32.1 ns | 36.6 ns |
| `rem@base` D307 | 37.2 ns | 43.5 ns | 42.9 ns | 42.5 ns | 42.5 ns |
| `rem@base` D462 | 74.1 ns | 92 ns | 74.2 ns | 48 ns | 55.4 ns |
| `rem@base` D616 | 91.2 ns | 88.3 ns | 62.4 ns | 84.6 ns | 93.8 ns |
| `rem@base` D924 | 76.4 ns | 113 ns | 96.8 ns | 90.6 ns | 75.7 ns |
| `rem@base` D1232 | 135 ns | 81.9 ns | 129 ns | 116 ns | 122 ns |

<div class="perf-chart"><input type="checkbox" class="fam-toggle" id="f-rem-0" checked><figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,191.9 88.2,170.4 124.4,149.5 160.5,148.2 196.7,132.8 232.9,129.0 269.1,113.1 305.3,105.3 341.5,85.3 377.6,79.3 413.8,84.5 450.0,68.0 450.0,70.9 413.8,84.7 377.6,78.5 341.5,93.7 305.3,101.4 269.1,105.8 232.9,137.9 196.7,132.8 160.5,164.4 124.4,152.9 88.2,194.9 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,170.4 124.4,149.5 160.5,148.2 196.7,132.8 232.9,129.0 269.1,113.1 305.3,105.3 341.5,85.3 377.6,79.3 413.8,84.5 450.0,68.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,169.5 124.4,149.5 160.5,161.5 196.7,132.7 232.9,137.3 269.1,110.1 305.3,100.8 341.5,79.1 377.6,80.3 413.8,73.2 450.0,82.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,173.8 124.4,156.8 160.5,167.1 196.7,151.4 232.9,145.3 269.1,107.9 305.3,101.2 341.5,85.3 377.6,90.3 413.8,77.6 450.0,69.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,180.9 124.4,168.0 160.5,164.3 196.7,140.1 232.9,145.2 269.1,109.5 305.3,101.4 341.5,97.9 377.6,81.5 413.8,79.5 450.0,72.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,194.9 124.4,152.9 160.5,164.4 196.7,132.8 232.9,137.9 269.1,105.8 305.3,101.4 341.5,93.7 377.6,78.5 413.8,84.7 450.0,70.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). solid = scale 0 and max, dashed = the intermediate band-edge scales.</figcaption><div class="fam-legend"><label class="fam-key k1" for="f-rem-0">rem@base</label></div></figure></div>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Row | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| `sub@base` D18 | 0.937 ns | 0.936 ns | 0.937 ns | 1.06 ns | 0.935 ns |
| `sub@base` D38 | 1.8 ns | 1.8 ns | 1.62 ns | 1.41 ns | 0.736 ns |
| `sub@base` D57 | 2.5 ns | 2.51 ns | 1.94 ns | 1.77 ns | 2.27 ns |
| `sub@base` D76 | 3.08 ns | 2.12 ns | 2 ns | 2.09 ns | 2.09 ns |
| `sub@base` D115 | 5.54 ns | 5.54 ns | 3.59 ns | 4.3 ns | 5.55 ns |
| `sub@base` D153 | 7.65 ns | 6.4 ns | 4.6 ns | 4.61 ns | 6.15 ns |
| `sub@base` D230 | 13.7 ns | 16 ns | 16.2 ns | 16.2 ns | 17.7 ns |
| `sub@base` D307 | 19.6 ns | 23.4 ns | 23.4 ns | 23.5 ns | 23.4 ns |
| `sub@base` D462 | 37.6 ns | 48.5 ns | 37.3 ns | 31.6 ns | 35.7 ns |
| `sub@base` D616 | 50.8 ns | 50.4 ns | 42.1 ns | 48.2 ns | 60.8 ns |
| `sub@base` D924 | 56.7 ns | 84.9 ns | 78.7 ns | 75 ns | 71.3 ns |
| `sub@base` D1232 | 95.3 ns | 64.1 ns | 106 ns | 95.5 ns | 106 ns |

<div class="perf-chart"><input type="checkbox" class="fam-toggle" id="f-sub-0" checked><figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,161.4 88.2,147.2 124.4,140.1 160.5,135.6 196.7,122.8 232.9,115.8 269.1,103.2 305.3,95.4 341.5,81.3 377.6,74.7 413.8,72.3 450.0,61.0 450.0,58.7 413.8,67.4 377.6,70.8 341.5,82.4 305.3,91.5 269.1,97.6 232.9,120.6 196.7,122.8 160.5,144.0 124.4,142.2 88.2,166.7 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,147.2 124.4,140.1 160.5,135.6 196.7,122.8 232.9,115.8 269.1,103.2 305.3,95.4 341.5,81.3 377.6,74.7 413.8,72.3 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,147.2 124.4,140.0 160.5,143.7 196.7,122.8 232.9,119.7 269.1,99.7 305.3,91.5 341.5,75.7 377.6,74.9 413.8,63.5 450.0,69.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.6 124.4,145.6 160.5,145.0 196.7,132.2 232.9,126.9 269.1,99.5 305.3,91.5 341.5,81.4 377.6,78.8 413.8,65.2 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,152.6 124.4,147.6 160.5,144.0 196.7,128.3 232.9,126.8 269.1,99.5 305.3,91.5 341.5,85.0 377.6,75.8 413.8,66.3 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,166.7 124.4,142.2 160.5,144.0 196.7,122.8 232.9,120.6 269.1,97.6 305.3,91.5 341.5,82.4 377.6,70.8 413.8,67.4 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). solid = scale 0 and max, dashed = the intermediate band-edge scales.</figcaption><div class="fam-legend"><label class="fam-key k1" for="f-sub-0">sub@base</label></div></figure></div>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
