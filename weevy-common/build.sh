cd $(dirname $0)
npx tsc -d ./index.ts -t esnext -m preserve --skipLibCheck --lib esnext,dom