#!/usr/bin/fish

set HERE (cd (dirname (status -f))/../; and pwd)
source $HERE/env.fish

function safeRm
	if test -L $argv[1]
		unlink $argv[1]
	else if test -e $argv[1]
		mv $argv[1] $argv[1].origin
	end
end

function withSu
	if test (id -u -n) = "root"
		eval $argv[1]
	else
		eval "sudo "$argv[1]
	end
end

function has
	return (type $argv[1] > /dev/null 2>&1)
end

function confirm
	set MSG $argv[1]
	while true
		read -P $MSG -n 1 ANS
		switch (echo $ANS)
			case y Y
				return 0
			case n N
				return 1
			case '*'
		end
	end
end
