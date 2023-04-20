echo $1

case $1 in

	"web")
		cargo build --target wasm32-unknown-unknown
		cp target/wasm32-unknown-unknown/debug/upsi3.wasm ./web/
	;;

	"run-web")
		basic-http-server ./web/
	;;

	*)
		echo "nothing"

esac