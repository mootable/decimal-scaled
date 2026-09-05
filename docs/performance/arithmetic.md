# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.06 ns | 1.05 ns | 1.14 ns | · | · |
| D38 | 1.62 ns | 1.63 ns | 1.62 ns | 1.62 ns | 1.62 ns |
| D57 | 1.21 ns | 2.25 ns | 2.26 ns | 1.43 ns | 1.1 ns |
| D76 | 3.09 ns | 3.5 ns | 1.8 ns | 3.1 ns | 3.49 ns |
| D115 | 4.4 ns | 4.42 ns | 3.33 ns | 4.39 ns | · |
| D153 | 5.89 ns | 3.8 ns | 5.95 ns | 5.89 ns | · |
| D230 | 15.6 ns | 11.9 ns | 15.3 ns | 14.6 ns | · |
| D307 | 18.5 ns | 14.3 ns | 16.1 ns | 18.5 ns | · |
| D462 | 38.7 ns | 28.8 ns | 29.5 ns | · | · |
| D616 | 69.1 ns | 45.3 ns | 48.6 ns | · | · |
| D924 | 72.6 ns | 83.8 ns | 86.7 ns | 82.5 ns | · |
| D1232 | 79.5 ns | 94.8 ns | 69.8 ns | 95 ns | · |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,207.6 88.2,189.0 124.4,201.7 160.5,161.0 196.7,145.7 232.9,133.0 269.1,90.8 305.3,83.2 341.5,51.3 377.6,26.1 413.8,23.9 450.0,20.0 160.5,155.7 124.4,205.7 88.2,189.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,207.6 88.2,189.0 124.4,201.7 160.5,161.0 196.7,145.7 232.9,133.0 269.1,90.8 305.3,83.2 341.5,51.3 377.6,26.1 413.8,23.9 450.0,20.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,207.7 88.2,188.9 124.4,174.8 160.5,155.6 196.7,145.5 232.9,152.0 269.1,102.3 305.3,94.4 341.5,64.1 377.6,44.3 413.8,17.7 450.0,12.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,204.4 88.2,189.0 124.4,174.6 160.5,184.5 196.7,157.8 232.9,132.5 269.1,91.4 305.3,89.3 341.5,63.0 377.6,41.3 413.8,16.2 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="88.2,189.0 124.4,194.4 160.5,160.8 196.7,145.7 232.9,133.0 269.1,93.7 305.3,83.2 413.8,18.4 450.0,12.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="88.2,189.0 124.4,205.7 160.5,155.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.19 ns | 8.79 ns | 8.31 ns | · | · |
| D38 | 9.98 ns | 11.4 ns | 58.8 ns | 59 ns | · |
| D57 | 17 ns | 33.2 ns | 70.6 ns | 84.8 ns | 72 ns |
| D76 | 26.4 ns | 65.3 ns | 50.9 ns | 113 ns | 150 ns |
| D115 | 83.9 ns | 106 ns | 183 ns | 232 ns | · |
| D153 | 54.9 ns | 74.8 ns | 144 ns | 320 ns | · |
| D230 | 171 ns | 196 ns | 438 ns | 597 ns | · |
| D307 | 220 ns | 276 ns | 531 ns | 892 ns | · |
| D462 | 261 ns | 407 ns | 1.35 µs | · | · |
| D616 | 664 ns | 1.04 µs | 1.84 µs | · | · |
| D924 | 326 ns | 1.14 µs | 1.77 µs | · | · |
| D1232 | 390 ns | 3.55 µs | 3.27 µs | 7.09 µs | · |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,170.4 88.2,160.0 124.4,148.4 160.5,138.9 196.7,113.8 232.9,123.0 269.1,98.4 305.3,92.9 341.5,89.2 377.6,68.9 413.8,84.4 450.0,80.4 160.5,101.2 124.4,117.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,170.4 88.2,160.0 124.4,148.4 160.5,138.9 196.7,113.8 232.9,123.0 269.1,98.4 305.3,92.9 341.5,89.2 377.6,68.9 413.8,84.4 450.0,80.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,162.8 88.2,157.1 124.4,134.0 160.5,119.3 196.7,108.7 232.9,116.3 269.1,95.4 305.3,88.0 341.5,79.5 377.6,59.2 413.8,57.2 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.0 88.2,121.5 124.4,117.6 160.5,124.7 196.7,96.9 232.9,102.1 269.1,77.9 305.3,73.7 341.5,53.4 377.6,46.8 413.8,47.7 450.0,34.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="88.2,121.5 124.4,113.6 160.5,107.3 196.7,91.8 232.9,84.7 269.1,71.2 305.3,62.5 450.0,17.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="124.4,117.1 160.5,101.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.07 ns | 5.01 ns | 4.31 ns | · | · |
| D38 | 3.51 ns | 14.3 ns | 27.2 ns | 26.4 ns | · |
| D57 | 2.98 ns | 21.4 ns | 34.2 ns | 53.6 ns | 40.1 ns |
| D76 | 8.1 ns | 38.2 ns | 30.8 ns | 78.7 ns | 108 ns |
| D115 | 50.9 ns | 84.4 ns | 180 ns | 229 ns | · |
| D153 | 18.1 ns | 33.6 ns | 113 ns | 357 ns | · |
| D230 | 132 ns | 285 ns | 567 ns | 971 ns | · |
| D307 | 170 ns | 353 ns | 850 ns | 1.39 µs | · |
| D462 | 121 ns | 415 ns | 2.46 µs | · | · |
| D616 | 723 ns | 1.74 µs | 2.74 µs | · | · |
| D924 | 131 ns | 1.49 µs | 2.64 µs | · | · |
| D1232 | 145 ns | 4.66 µs | 6.03 µs | 12.9 µs | · |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.5 88.2,188.2 124.4,191.0 160.5,173.7 196.7,141.7 232.9,159.7 269.1,125.1 305.3,120.8 341.5,126.6 377.6,95.6 413.8,125.2 450.0,123.6 160.5,128.6 124.4,145.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.5 88.2,188.2 124.4,191.0 160.5,173.7 196.7,141.7 232.9,159.7 269.1,125.1 305.3,120.8 341.5,126.6 377.6,95.6 413.8,125.2 450.0,123.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,182.0 88.2,163.7 124.4,156.8 160.5,146.7 196.7,132.9 232.9,149.0 269.1,111.8 305.3,108.1 341.5,105.3 377.6,80.4 413.8,83.1 450.0,63.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.6 88.2,152.6 124.4,148.6 160.5,150.5 196.7,119.8 232.9,127.9 269.1,99.8 305.3,92.8 341.5,74.4 377.6,72.5 413.8,73.1 450.0,58.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="88.2,153.1 124.4,140.8 160.5,134.2 196.7,115.6 232.9,107.9 269.1,90.5 305.3,84.3 450.0,45.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="124.4,145.9 160.5,128.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.704 ns | 0.703 ns | 0.605 ns | · | · |
| D38 | 1.33 ns | 1.33 ns | 1.33 ns | 1.33 ns | · |
| D57 | 1.05 ns | 1.91 ns | 1.26 ns | 0.924 ns | · |
| D76 | 2.18 ns | 2.16 ns | 1.45 ns | 2.18 ns | 2.63 ns |
| D115 | 2.83 ns | 2.83 ns | 2.78 ns | 3.29 ns | · |
| D153 | 3.79 ns | 2.72 ns | 4.3 ns | 4.29 ns | · |
| D230 | 6.65 ns | 5.77 ns | 7.43 ns | 7.06 ns | · |
| D307 | 11 ns | 6.76 ns | 7.74 ns | 11 ns | · |
| D462 | 20.3 ns | 15.1 ns | 15 ns | · | · |
| D616 | 33.9 ns | 20.2 ns | 20.1 ns | · | · |
| D924 | 55 ns | 81.4 ns | 88.6 ns | · | · |
| D1232 | 38 ns | 61.5 ns | 46.9 ns | 61.4 ns | · |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,153.5 88.2,135.2 124.4,141.9 160.5,120.7 196.7,113.2 232.9,104.8 269.1,88.5 305.3,73.9 341.5,56.2 377.6,41.3 413.8,27.3 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.5 88.2,135.2 124.4,124.5 160.5,121.0 196.7,113.2 232.9,114.4 269.1,92.6 305.3,88.0 341.5,64.7 377.6,56.3 413.8,16.0 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.9 88.2,135.2 124.4,136.6 160.5,132.5 196.7,113.7 232.9,101.1 269.1,85.3 305.3,84.1 341.5,65.0 377.6,56.5 413.8,13.5 450.0,31.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="88.2,135.1 124.4,145.6 160.5,120.7 196.7,108.9 232.9,101.2 269.1,86.7 305.3,73.9 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.11 ns | 2.46 ns | 3.23 ns | · | · |
| D38 | 3.44 ns | 3.44 ns | 3.21 ns | 3.49 ns | · |
| D57 | 3.7 ns | 7.34 ns | 3.68 ns | 3.25 ns | · |
| D76 | 8.72 ns | 9.52 ns | 4.83 ns | 8.43 ns | 9.77 ns |
| D115 | 12.7 ns | 12.7 ns | 9.12 ns | 12.7 ns | · |
| D153 | 16.5 ns | 10.9 ns | 17.6 ns | 17.7 ns | · |
| D230 | 36.3 ns | 28.1 ns | 36.2 ns | 34.2 ns | · |
| D307 | 40.9 ns | 28.4 ns | 31.9 ns | 42.5 ns | · |
| D462 | 107 ns | 73.6 ns | 71.2 ns | · | · |
| D616 | 119 ns | 79.8 ns | 78.1 ns | · | · |
| D924 | 93 ns | 98.1 ns | 115 ns | · | · |
| D1232 | 98.2 ns | 120 ns | 73.4 ns | 121 ns | · |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,188.4 88.2,174.2 124.4,172.1 160.5,147.3 196.7,136.3 232.9,128.8 269.1,106.0 305.3,102.5 341.5,74.7 377.6,71.6 413.8,78.8 450.0,77.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,183.9 88.2,174.2 124.4,152.3 160.5,144.8 196.7,136.3 232.9,140.8 269.1,113.4 305.3,113.2 341.5,85.5 377.6,83.2 413.8,77.2 450.0,71.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,176.1 88.2,176.3 124.4,172.3 160.5,164.4 196.7,146.0 232.9,126.9 269.1,106.1 305.3,109.8 341.5,86.5 377.6,83.8 413.8,72.6 450.0,85.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="88.2,173.8 124.4,175.8 160.5,148.3 196.7,136.3 232.9,126.7 269.1,107.7 305.3,101.4 450.0,71.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.06 ns | 1.06 ns | 1.12 ns | · | · |
| D38 | 1.61 ns | 1.6 ns | 1.61 ns | 1.62 ns | · |
| D57 | 1.3 ns | 2.28 ns | 2.38 ns | 1.52 ns | 1.15 ns |
| D76 | 3.09 ns | 3.45 ns | 2.09 ns | 3.09 ns | 3.46 ns |
| D115 | 4.89 ns | 4.88 ns | 4.03 ns | 4.88 ns | · |
| D153 | 7.64 ns | 5.11 ns | 7.68 ns | 7.61 ns | · |
| D230 | 17.7 ns | 13.7 ns | 17.8 ns | 16.3 ns | · |
| D307 | 23 ns | 16.6 ns | 18.5 ns | 23.1 ns | · |
| D462 | 50.3 ns | 36.8 ns | 37.1 ns | · | · |
| D616 | 69.2 ns | 45.9 ns | 49.6 ns | · | · |
| D924 | 72.6 ns | 83.7 ns | 92.8 ns | 84.2 ns | · |
| D1232 | 88.8 ns | 95.3 ns | 74.5 ns | 95.5 ns | · |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,207.4 88.2,189.4 124.4,198.7 160.5,161.0 196.7,141.1 232.9,121.7 269.1,85.3 305.3,73.8 341.5,39.8 377.6,26.0 413.8,23.9 450.0,15.2 160.5,156.1 124.4,204.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,207.4 88.2,189.4 124.4,198.7 160.5,161.0 196.7,141.1 232.9,121.7 269.1,85.3 305.3,73.8 341.5,39.8 377.6,26.0 413.8,23.9 450.0,15.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,207.4 88.2,189.5 124.4,174.3 160.5,156.2 196.7,141.1 232.9,139.2 269.1,96.2 305.3,88.0 341.5,53.5 377.6,43.8 413.8,17.7 450.0,12.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,205.0 88.2,189.2 124.4,172.3 160.5,178.0 196.7,149.4 232.9,121.5 269.1,85.0 305.3,83.3 341.5,53.0 377.6,40.4 413.8,13.2 450.0,22.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="88.2,189.1 124.4,191.7 160.5,161.0 196.7,141.1 232.9,121.9 269.1,88.9 305.3,73.6 413.8,17.4 450.0,12.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="124.4,204.0 160.5,156.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
