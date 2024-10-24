import pandas as pd

from vpn_config_parser import Declare

def flatten_declare_objects(declares: list[Declare]) -> pd.DataFrame:
    """
    Преобразует список объектов Declare в плоский DataFrame, где путь является частью ключа.
    
    Args:
        declares: Список объектов Declare
        
    Returns:
        pd.DataFrame: DataFrame с колонками 'name', 'key', 'type', 'value'
    """
    flattened_data = []
    
    def process_declare(declare: Declare, parent_path: str = "", root_name: str = "") -> None:
        """
        Рекурсивно обрабатывает объект Declare и его вложенные declarations.
        
        Args:
            declare: Объект класса Declare
            parent_path: Путь к текущему параметру в иерархии
            root_name: Имя корневого пользователя
        """
        # Определяем корневое имя при первом вызове
        current_root = root_name if root_name else declare.name
        
        # Формируем текущий путь
        current_path = f"{parent_path}.{declare.name}" if parent_path else declare.name
        
        # Для констант корневого уровня не добавляем путь к ключу
        for constant in declare.constants:
            if not parent_path:
                key = constant.key
            else:
                # Убираем корневое имя из пути и добавляем к ключу
                path_without_root = current_path[len(current_root):].lstrip('.')
                key = f"{path_without_root}.{constant.key}"
            
            flattened_data.append({
                'name': current_root,
                'key': key,
                'type': constant.type,
                'value': constant.value
            })
        
        # Рекурсивно обрабатываем вложенные declarations
        for subdeclare in declare.declarations:
            process_declare(subdeclare, current_path, current_root)
    
    # Обрабатываем каждый объект Declare
    for declare in declares:
        process_declare(declare)
    
    # Создаем DataFrame
    df = pd.DataFrame(flattened_data)
    
    # Если данных нет, возвращаем пустой DataFrame с нужными колонками
    if df.empty:
        return pd.DataFrame(columns=['name', 'key', 'type', 'value'])
    
    # Сортируем для лучшей читаемости
    return df.sort_values(['name', 'key']).reset_index(drop=True)