start=$(date -v -1m +%s)
end=$(date -v -1d +%s)
fail_count=0
while [ $start -le $end ]
do
  d=$(date -r $start +%Y-%m-%d)
  if ! curl -sf "http://localhost:8000/api/pull?date=$d" >/dev/null 2>&1; then
    echo "Failed to pull data for date: $d"
  fi
  start=$((start + 86400))
  sleep 3
done
