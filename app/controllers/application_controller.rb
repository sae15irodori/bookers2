class ApplicationController < ActionController::Base
  class ApplicationController < ActionController::Base
  before_action :configure_permitted_parameters, if: :devise_controller?
  #ユーザー認証・登録する前に(devise利用する前)↓のメソッド実行される
  
    def after_sign_in_path_for(resource)
      about_path#ここは後程変更
    end
    
    
    def after_sign_aout_path_for(resource)
      about_path#ここは後程変更
    end
  protected

  def configure_permitted_parameters
    devise_parameter_sanitizer.permit(:sign_up, keys: [:email])#
  end
  end
end
