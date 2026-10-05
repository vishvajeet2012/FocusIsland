<script lang="ts">
  import { onMount } from "svelte";
  import { getInsights } from "../lib/native";
  import { formatDuration } from "../lib/time";
  import type { Insights } from "../types/app";
  import Icon from "../components/common/Icon.svelte";

  let loading = true;
  let error = "";
  let insights: Insights | null = null;

  onMount(async () => {
    try { insights = await getInsights(); }
    catch (reason) { error = reason instanceof Error ? reason.message : "Insights unavailable"; }
    finally { loading = false; }
  });

  $: maximumMinutes = Math.max(1, ...(insights?.days.map((day) => day.focusMinutes) ?? [1]));
</script>

<div class="insights-view">
  {#if loading}
    <div class="state"><span class="spinner"></span><p>Calculating your week…</p></div>
  {:else if error}
    <div class="state"><span class="state-icon">!</span><p>{error}</p></div>
  {:else if insights}
    <section class="metric focus-metric">
      <span class="metric-icon"><Icon name="clock" size={15} /></span>
      <p>Today’s focus</p>
      <strong>{formatDuration(insights.todayFocusSeconds)}</strong>
      <small>{insights.sessionsToday} {insights.sessionsToday === 1 ? "session" : "sessions"}</small>
    </section>
    <section class="metric tasks-metric">
      <span class="metric-icon"><Icon name="check" size={15} /></span>
      <p>Tasks completed</p>
      <strong>{insights.tasksCompletedToday}</strong>
      <small>today</small>
    </section>
    <section class="metric streak-metric">
      <span class="metric-icon"><Icon name="spark" size={15} /></span>
      <p>Current streak</p>
      <strong>{insights.currentStreak}</strong>
      <small>{insights.currentStreak === 1 ? "day" : "days"}</small>
    </section>
    <section class="chart-card">
      <header><div><h2>Last 7 days</h2><p>Focus minutes</p></div><strong>{insights.weeklyFocusMinutes}<small> min</small></strong></header>
      {#if insights.weeklyFocusMinutes === 0}
        <div class="chart-empty"><Icon name="spark" size={18} /><span>Complete your first focus session to see insights.</span></div>
      {:else}
        <svg viewBox="0 0 420 154" role="img" aria-label="Focus minutes over the last seven days" preserveAspectRatio="none">
          <line x1="6" y1="118" x2="414" y2="118" />
          {#each insights.days as day, index}
            {@const height = Math.max(day.focusMinutes > 0 ? 5 : 0, (day.focusMinutes / maximumMinutes) * 98)}
            <g>
              <rect x={16 + index * 58} y={118 - height} width="34" height={height} rx="7"><title>{day.label}: {day.focusMinutes} minutes</title></rect>
              <text x={33 + index * 58} y="141" text-anchor="middle">{day.label.slice(0, 2)}</text>
            </g>
          {/each}
        </svg>
      {/if}
    </section>
    <section class="details-card">
      <h2>Weekly rhythm</h2>
      <dl>
        <div><dt>Most productive</dt><dd>{insights.mostProductiveDay ?? "—"}</dd></div>
        <div><dt>Average session</dt><dd>{insights.averageSessionSeconds ? formatDuration(insights.averageSessionSeconds) : "—"}</dd></div>
        <div><dt>Completed tasks</dt><dd>{insights.weeklyCompletedTasks}</dd></div>
        <div><dt>Total focus</dt><dd>{formatDuration(insights.weeklyFocusMinutes * 60)}</dd></div>
      </dl>
    </section>
  {/if}
</div>

<style>
  .insights-view { display: grid; grid-template-columns: repeat(3, minmax(0, .72fr)) minmax(260px, 1.45fr); grid-template-rows: 105px minmax(0, 1fr); gap: 8px; height: 100%; color: var(--shell-text); }
  section { border: 1px solid var(--surface-border); border-radius: 18px; background: var(--surface-bg); }
  .metric { position: relative; display: grid; align-content: end; padding: 13px; overflow: hidden; }
  .metric::after { content: ""; position: absolute; right: -22px; top: -26px; width: 75px; height: 75px; border-radius: 50%; background: var(--surface-control); }
  .metric-icon { position: absolute; top: 11px; left: 12px; display: grid; place-items: center; width: 27px; height: 27px; border-radius: 9px; background: var(--surface-selected); }
  .focus-metric { background: var(--metric-focus); } .tasks-metric { background: var(--metric-tasks); } .streak-metric { background: var(--metric-streak); }
  .metric p { margin: 0 0 2px; color: var(--shell-muted); font-size: 9px; }
  .metric strong { font-size: 22px; line-height: 1; letter-spacing: -.7px; }
  .metric small { position: absolute; right: 11px; bottom: 14px; color: var(--shell-muted); font-size: 8px; }
  .chart-card { grid-column: 1 / 4; display: grid; grid-template-rows: auto 1fr; min-height: 0; padding: 14px; }
  .chart-card header { display: flex; justify-content: space-between; align-items: flex-start; }
  h2 { margin: 0; font-size: 12px; }
  .chart-card p { margin: 2px 0 0; color: var(--shell-subtle); font-size: 8px; }
  .chart-card header > strong { font-size: 15px; } .chart-card header small { color: var(--shell-subtle); font-size: 8px; font-weight: 500; }
  svg { width: 100%; height: 100%; min-height: 110px; overflow: visible; }
  svg line { stroke: var(--surface-line); stroke-width: 1; vector-effect: non-scaling-stroke; }
  svg rect { fill: #b7d0bb; opacity: .88; transition: opacity var(--motion-fast) ease; }
  svg rect:hover { opacity: 1; }
  svg text { fill: var(--shell-subtle); font: 8px "Segoe UI", sans-serif; }
  .chart-empty { display: flex; align-items: center; justify-content: center; gap: 8px; color: var(--shell-subtle); font-size: 9px; }
  .details-card { grid-column: 4; grid-row: 1 / 3; padding: 15px; }
  .details-card dl { display: grid; gap: 0; margin: 15px 0 0; }
  .details-card dl div { display: flex; justify-content: space-between; gap: 12px; padding: 12px 0; border-bottom: 1px solid var(--surface-line); }
  .details-card dl div:last-child { border: 0; }
  dt { color: var(--shell-muted); font-size: 9px; } dd { margin: 0; font-size: 10px; font-weight: 650; text-align: right; }
  .state { grid-column: 1 / -1; display: grid; place-items: center; align-content: center; gap: 8px; color: var(--shell-muted); font-size: 10px; }
  .state p { margin: 0; }
  .spinner { width: 20px; height: 20px; border: 2px solid rgba(255,255,255,.12); border-top-color: rgba(255,255,255,.65); border-radius: 50%; animation: spin .8s linear infinite; }
  .state-icon { display: grid; place-items: center; width: 25px; height: 25px; border-radius: 50%; background: rgba(220,150,145,.18); }
  @keyframes spin { to { transform: rotate(1turn); } }
</style>
