interface ClimateScreen {
  thresholdsMm: number[];
  commonValidCells: number;
  rows: {yearsBP: number; fractions: number[]}[];
}
async function loadEvidence() {
  const status = document.getElementById('climate-status')!;
  const select = document.getElementById('climate-year') as HTMLSelectElement;
  const body = document.getElementById('climate-results')!;
  try {
    const response = await fetch('/climate-screen.json');
    if (!response.ok) throw new Error('Climate data unavailable. Reload to retry.');
    const data: ClimateScreen = await response.json();
    select.replaceChildren(...data.rows.map((row, i) => new Option(`${row.yearsBP.toLocaleString()} years BP`, String(i))));
    select.disabled = false;
    const update = () => {
      const row = data.rows[Number(select.value)];
      body.replaceChildren(...data.thresholdsMm.map((threshold, i) => {
        const tr = document.createElement('tr');
        const th = document.createElement('th'); th.scope = 'row'; th.textContent = String(threshold);
        const td = document.createElement('td'); td.textContent = `${(row.fractions[i] * 100).toFixed(1)}%`;
        tr.append(th, td); return tr;
      }));
      status.textContent = `${data.commonValidCells.toLocaleString()} common cells · 0.5° grid · BP relative to 1950 · 41 dates, 120,000–40,000 BP`;
    };
    select.addEventListener('change', update);
    update();
  } catch (error) { status.textContent = error instanceof Error ? error.message : 'Climate data could not load.'; }
}
void loadEvidence();
