#!/usr/bin/env python3

from copy import deepcopy


def matching_brace(expr, opening_index):
    braces_open = 1
    i = opening_index
    while braces_open != 0:
        i += 1
        if expr[i] == '(':
            braces_open += 1
        elif expr[i] == ')':
            braces_open -= 1
    return i


def parse_expr(line):
    expr = []
    i = 0
    while i < len(line):
        if line[i] in ['+', '*']:
            expr.append(line[i])
            i += 1
        elif line[i] == '(':
            match_i = matching_brace(line, i)
            expr.append(parse_expr(line[i + 1:match_i]))
            i = match_i + 1
        else:
            expr.append(int(line[i]))
            i += 1
    return expr


def read_expr(line):
    return parse_expr(line.strip().replace('(', '( ').replace(')', ' )').split(' '))


def evaluate1(expr):
    if isinstance(expr, int):
        return expr
    elif len(expr) == 1:
        return expr[0]

    if expr[1] == '*':
        expr[:3] = [evaluate1(expr[0]) * evaluate1(expr[2])]
    elif expr[1] == '+':
        expr[:3] = [evaluate1(expr[0]) + evaluate1(expr[2])]
    else:
        assert False

    return evaluate1(expr)


def evaluate2(expr):
    if isinstance(expr, int):
        return expr
    elif len(expr) == 1:
        return expr[0]

    if '+' in expr:
        sign_index = expr.index('+')
        expr[sign_index - 1:sign_index + 2] = \
            [evaluate2(expr[sign_index - 1]) + evaluate2(expr[sign_index + 1])]
    elif expr[1] == '*':
        expr[:3] = [evaluate2(expr[0]) * evaluate2(expr[2])]
    else:
        assert False

    return evaluate2(expr)


def get_solution(exprs, eval_func):
    return sum(eval_func(deepcopy(expr)) for expr in exprs)


if __name__ == '__main__':
    with open('input') as f:
        exprs = [read_expr(line) for line in f]

    print(get_solution(exprs, evaluate1))
    print(get_solution(exprs, evaluate2))
